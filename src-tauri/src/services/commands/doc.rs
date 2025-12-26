use crate::config::Config;
use crate::dtos::ProgressWrapper;
pub(crate) use crate::dtos::doc::{DocumentTags, PartDoc};
use crate::entities::prelude::Document;
use crate::services::commands::doc::doc_util::{handle_many_paths, handle_query_docs_by_tags, suggest_tag, update_paper_summary};
use crate::services::curd::document::DocumentCurd;
use log::{error, info};
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use clipboard_rs::{Clipboard, ClipboardContext};
use tauri::{AppHandle, Emitter, State};
use crate::dtos::tag::TagAndGroups;

#[tauri::command]
pub async fn insert_docs(
    config: State<'_, Mutex<Config>>,
    //我如果把ai放在State中，会报错，因为只有引用，但是我需要发到协程里面使用，活的不够长，改成'static也有错误。
    //用Option再take的话没有意义了，因为后面还要用，不如直接用OnceLock装起来。
    // o_ai: State<'_, tokio::sync::Mutex<Option<AI>>>,
    // o_ai: State<'static, tokio::sync::Mutex<Option<AI>>>,
    app_handle: AppHandle,
    paths: Vec<String>,
    tags_id: Vec<i32>,
) -> Result<(), String> {
    let flag = { config.lock().unwrap().ai_config.use_ai };
    let max_concurrency = { config.lock().unwrap().ai_config.max_concurrency };
    match handle_many_paths(flag, max_concurrency, app_handle, paths, tags_id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("插入文档失败：{}", e);
            Err("插入文档失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn query_docs_by_tags(
    and_tags_id: Vec<i32>,
    or_tags_id: Vec<i32>,
) -> Result<Vec<DocumentTags>, String> {
    match handle_query_docs_by_tags(and_tags_id, or_tags_id).await {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("查找文档失败：{}", e);
            Err("查找文档失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn delete_docs(ids: Vec<i32>) -> Result<(), String> {
    match DocumentCurd::delete_many(ids).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("删除文档失败：{}", e);
            Err("删除文档失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn update_doc_detail(app_handle: AppHandle, doc: Document) -> Result<(), String> {
    let id = doc.id;
    match DocumentCurd::update_detail(doc).await {
        Ok(_) => {
            let document_tags = DocumentTags::from_doc_id(id).await;
            let _ = app_handle.emit("doc_update", document_tags);
            Ok(())
        }
        Err(e) => {
            error!("更新文档失败：{}", e);
            Err("更新文档失败".to_string())
        }
    }
}
/// 总结文档
#[tauri::command]
pub async fn summarize_docs_by_ai(
    app_handle: AppHandle,
    document_tags_list: Vec<DocumentTags>,
) -> Result<(), String> {
    let a_pro = Arc::new(ProgressWrapper::new(
        "正在总结文档",
        document_tags_list.len() as i32,
    ));
    let _ = app_handle.emit("progress_event", a_pro.get_progress());
    for document_tags in document_tags_list {
        let _ = app_handle.emit("progress_event", a_pro.update("正在总结文档", 0));
        let _ = app_handle.emit("summary_doc", (document_tags.id, true));
        info!("正在总结文档：{}", document_tags.title);
        //有的pdf会提取文字失败导致panic（https://github.com/jrmuizel/adobe-cmap-parser/issues/2）
        // 然后前端会获得Failed to load resource: net::ERR_CONNECTION_REFUSED
        //然后不知道为什么后端会再次运行这个函数（前端是没问题的，只调用了一次），导致再次触发一次panic。
        match update_paper_summary(&document_tags.path, document_tags.id).await {
            Ok(new_document_tags) => {
                let _ = app_handle.emit("summary_doc", (document_tags.id, false));
                let _ = app_handle.emit("doc_update", new_document_tags);
                let _ = app_handle.emit("progress_event", a_pro.update("总结文档完成", 1));
            }
            Err(e) => {
                error!("总结文档{}失败：{}", document_tags.title, e);
                let _ = app_handle.emit("summary_doc", (document_tags.id, false));
                let _ = app_handle.emit("progress_event", a_pro.update("总结文档失败", 1));
                return Err(e.to_string());
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn suggest_tag_by_ai(app_handle: AppHandle,path_s: &str,doc_id: i32,tag_and_groups: Vec<TagAndGroups> ) -> Result<(), String> {
    info!("建议标签：{}", path_s);
    match suggest_tag(path_s,doc_id,tag_and_groups).await {
        Ok(doc_tags) => {
            let _ = app_handle.emit("doc_update", doc_tags);
            // println!("建议标签：{:?}", tag_and_groups);
            Ok(())
        }
        Err(e) => {
            let msg = format!("建议标签失败：{}", e);
            error!("{}", msg);
            Err(msg)
        }
    }
}
#[tauri::command]
pub async fn open_by_system(path: String) -> Result<(), String> {
    match open::that(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("打开文档失败：{}", e);
            Err("打开文档失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn open_by_app(config: State<'_, Mutex<Config>>, path: String) -> Result<(), String> {
    let exes = &config.lock().unwrap().exe_configs;
    if let Some(exe) = exes.iter().find(|exe| exe.is_default) {
        info!("使用默认的打开方式：{}", exe.name);
        if let Err(e) = open::with(path, exe.path.clone()) {
            error!("打开文档失败：{}", e);
            Err("打开文档失败".to_string())
        } else {
            Ok(())
        }
    } else {
        error!("请先设置默认的打开方式");
        Err("请先设置默认的打开方式".to_string())
    }
}
/** 打开文档所在目录,注意传进来的是文件的路径，所以要读取父目录 **/
#[tauri::command]
pub async fn open_dir(path: String) -> Result<(), String> {
    match open::that(PathBuf::from(path).parent().unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("打开文档所在目录失败：{}", e);
            Err("打开文档所在目录失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn open_with_exe(exe_path: String, file_path: String) -> Result<(), String> {
    // 使用指定的可执行文件打开文件
    match open::with(file_path, exe_path) {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("使用指定的可执行文件打开文件失败：{}", err);
            Err("使用指定的可执行文件打开文件失败".to_string())
        }
    }
}

#[tauri::command]
pub async fn copy_files_to_clipboard(file_paths: Vec<String>) -> Result<(), String> {
    // 创建剪贴板上下文
    match ClipboardContext::new(){
        Ok(ctx) => {
            // 将文件 URI 列表写入剪贴板
            match ctx.set_files(file_paths){
                Ok(_) => {
                    info!("文件已复制到剪贴板");
                    Ok(())
                }
                Err(e) => {
                    error!("将文件 URI 列表写入剪贴板失败：{}", e);
                    Err("将文件 URI 列表写入剪贴板失败".to_string())
                }
            }
        }
        Err(e) => {
            error!("创建剪贴板上下文失败：{}", e);
            Err("创建剪贴板上下文失败".to_string())
        }
    }
}
#[allow(dead_code)]
mod doc_util {
    use crate::app_errors::AppError::Tip;
    use crate::app_errors::AppResult;
    use crate::dtos::ProgressWrapper;
    use crate::dtos::doc::{DocumentTags, PartDoc, TagResponse};
    use crate::entities::prelude::Document;
    use crate::services::ai::ONCE_AI;
    use crate::services::curd::doc_and_tag::DocAndTagCurd;
    use crate::services::curd::document::DocumentCurd;
    use crate::services::util::file::{extract_limit_pages, organize_files};
    use log::{error, info};
    use std::collections::{HashMap, HashSet};
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use tauri::Emitter;
    use tokio::sync::Semaphore;
    use tracing::{instrument};
    use crate::dtos::tag::TagAndGroups;

    /// 处理多个文件路径，执行文档插入、组织和更新操作
    ///
    /// # 功能描述
    ///
    /// 该函数主要负责处理传入的多个文件路径，执行以下操作：
    /// 1. 插入文档到数据库
    /// 2. 组织文件到按时间命名的文件夹
    /// 3. 更新文档路径
    /// 4. 发送进度更新和错误消息到前端
    ///
    /// # 操作过程
    ///
    /// 1. 创建信号量以限制最大并发数
    /// 2. 初始化进度包装器并发送初始进度更新
    /// 3. 遍历文件路径列表：
    ///    a. 检查文件是否存在，跳过不存在的文件
    ///    b. 跳过文件夹（目前不支持解析文件夹）
    ///    c. 提取文件名并插入文档到数据库
    ///    d. 如果有标签ID，插入文档和标签的关系
    ///    e. 根据文件扩展名处理不同类型的文件（如PDF）
    ///    f. 对于PDF文件，如果启用了AI，异步提取总结信息
    ///    g. 更新进度和发送相关事件到前端
    /// 4. 组织文件到按时间命名的文件夹
    /// 5. 更新数据库中的文档路径
    /// 6. 发送最终的进度更新和文档路径更新事件到前端
    ///
    /// # 参数
    ///
    /// * `use_ai` - 是否使用AI进行文档总结
    /// * `max_concurrency` - 最大并发数
    /// * `app_handle` - Tauri应用句柄，用于与前端通信
    /// * `paths` - 文件路径列表
    /// * `tags_id` - 标签ID列表
    ///
    /// # 返回值
    ///
    /// * `AppResult<()>` - 如果操作成功则返回`Ok(())`，否则返回错误，这个返回值可以忽略，因为信息都通过emit发送了。
    pub(crate) async fn handle_many_paths(
        use_ai: bool,
        max_concurrency: i32,
        // o_ai: &Option<AI>,
        // o_ai: &'static Option<AI>,
        // o_ai: Option<AI>,
        // o_ai: MutexGuard<Option<AI>>,
        app_handle: tauri::AppHandle,
        paths: Vec<String>,
        tags_id: Vec<i32>,
    ) -> AppResult<()> {
        // 创建一个信号量，限制最大并发数为 max_concurrency
        let semaphore = Arc::new(Semaphore::new(max_concurrency as usize));
        let total = paths.len() as i32;
        let app_handle = Arc::new(app_handle);
        let progress_wrapper = Arc::new(ProgressWrapper::new("正在插入文档", total));
        let _ = app_handle.emit("progress_event", progress_wrapper.get_progress());
        let files = Arc::new(Mutex::new(HashMap::with_capacity(total as usize)));
        for path_s in paths {
            let path = Path::new(&path_s);
            if !path.exists() {
                //虽然感觉不可能不存在的情况
                error!("路径不存在: {}", path.display());
                let _ = app_handle.emit("backend_message", "路径不存在");
                let _ = app_handle.emit(
                    "progress_event",
                    progress_wrapper.clone().update("路径不存在", 1),
                );
                continue;
            }
            if path.is_dir() {
                error!("不支持解析文件夹: {}", path.display());
                let _ = app_handle.emit("backend_message", "不支持解析文件夹");
                let _ = app_handle.emit(
                    "progress_event",
                    progress_wrapper.clone().update("不支持解析文件夹", 1),
                );
                continue;
            }
            // path.file_stem()
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    info!("File name: {}", file_name_str);
                    match DocumentCurd::insert(Document::new(file_name_str.into(), path_s.clone()))
                        .await
                    {
                        Ok(doc) => {
                            if !tags_id.is_empty() {
                                DocAndTagCurd::insert_many(doc.id, tags_id.clone())
                                    .await
                                    .expect("插入文档和标签关系失败");
                            }
                            let document_tags = DocumentTags::from_doc(doc).await;
                            info!("插入文档成功: {:?}", document_tags);
                            let id = document_tags.id;
                            let _ = app_handle.emit("insert_doc", document_tags);
                            if let Some(ext) = path.extension() {
                                match ext.to_str().unwrap() {
                                    "pdf" => {
                                        files.lock().unwrap().insert(id, path_s.clone());
                                        // 处理PDF文件,提取作者名、标题、摘要等信息
                                        if use_ai {
                                            info!("正在总结PDF文件: {}", path.display());
                                            let app_handle = app_handle.clone();
                                            let semaphore = semaphore.clone();
                                            let progress_wrapper = progress_wrapper.clone();
                                            tokio::spawn(async move {
                                                let permit = semaphore.acquire().await.unwrap();
                                                let _guard = permit;
                                                let _ = app_handle.emit(
                                                    "progress_event",
                                                    progress_wrapper.update("正在解析文档", 0),
                                                );
                                                let _ = app_handle.emit("summary_doc", (id, true));
                                                match update_paper_summary(&path_s, id).await {
                                                    Ok(document_tags) => {
                                                        let _ = app_handle
                                                            .emit("summary_doc", (id, false));
                                                        let _ = app_handle
                                                            .emit("doc_update", document_tags);
                                                        let _ = app_handle.emit(
                                                            "progress_event",
                                                            progress_wrapper
                                                                .clone()
                                                                .update("解析文档完成", 1),
                                                        );
                                                    }
                                                    Err(e) => {
                                                        error!("{}", e);
                                                        let _ = app_handle
                                                            .emit("summary_doc", (id, false));
                                                        let _ = app_handle.emit(
                                                            "backend_message",
                                                            "更新文档总结失败",
                                                        );
                                                        let _ = app_handle.emit(
                                                            "progress_event",
                                                            progress_wrapper
                                                                .clone()
                                                                .update("解析文档出现错误", 1),
                                                        );
                                                        // update_progress(progress_id,app_handle, "解析文档出现错误",1, count, total).await;
                                                    }
                                                }
                                            });
                                        } else {
                                            let _ = app_handle.emit(
                                                "progress_event",
                                                progress_wrapper.clone().update("插入文档完成", 1),
                                            );
                                        }
                                    }
                                    _ => {
                                        let _ = app_handle.emit(
                                            "progress_event",
                                            progress_wrapper.clone().update("插入完成", 1),
                                        );
                                    }
                                }
                            } else {
                                let _ = app_handle.emit(
                                    "progress_event",
                                    progress_wrapper.clone().update("插入完成", 1),
                                );
                            }
                        }
                        Err(e) => {
                            error!("插入文档{}失败: {}", path.display(), e);
                            let _ = app_handle.emit("backend_message", "插入文档失败");
                            let _ = app_handle.emit(
                                "progress_event",
                                progress_wrapper.clone().update("出现错误", 1),
                            );
                        }
                    }
                } else {
                    let err_msg = format!("无效的文件名: {}", path.display());
                    error!("{}", err_msg);
                    let _ = app_handle.emit("backend_message", err_msg);
                    let _ = app_handle.emit(
                        "progress_event",
                        progress_wrapper.clone().update("出现错误", 1),
                    );
                }
            } else {
                //文件夹的情况
                let err_msg = format!("未获取到文件名: {}", path.display());
                error!("{}", err_msg);
                let _ = app_handle.emit("backend_message", err_msg);
                let _ = app_handle.emit(
                    "progress_event",
                    progress_wrapper.clone().update("出现错误", 1),
                );
            }
        }
        let progress_wrapper = Arc::new(ProgressWrapper::new("正在复制文档", total));
        let mut files_guard = files.lock().unwrap().clone();
        match organize_files(&mut files_guard).await{
            Ok(_) => {
                let _ = DocumentCurd::update_paths_by_ids(&files_guard).await;
                let _ = app_handle.emit(
                    "progress_event",
                    progress_wrapper.clone().update("整理文档完成", total),
                );
                //只更新了路径，如果用let _ = app_handle.emit("doc_update", document_tags);前端更新性能差后端麻烦
                let _ = app_handle
                    .emit("docs_path_update",files_guard.clone());
            }
            Err(e) => {
                error!("整理文档失败: {}", e);
                let _ = app_handle.emit("backend_message", "整理文档失败");
                let _ = app_handle.emit(
                    "progress_event",
                    progress_wrapper.clone().update("出现错误", total),
                );
            }
        }
        Ok(())
    }
    #[instrument]
    pub async fn handle_query_docs_by_tags(
        and_tags_id: Vec<i32>,
        or_tags_id: Vec<i32>,
    ) -> AppResult<Vec<DocumentTags>> {
        //这个是包含所有必须标签的文档
        let and_docs = DocAndTagCurd::find_documents_with_tags(and_tags_id, true).await?;
        // info!("查询到并集{:?}文档", and_docs);
        //这个是包含至少一个标签的文档，一般数量会大于and_docs
        let or_docs = DocAndTagCurd::find_documents_with_tags(or_tags_id, false).await?;
        // info!("查询到或集{:?}文档", or_docs);
        // 计算交集
        let common_docs = and_docs
            .into_iter()
            .collect::<HashSet<_>>()
            .intersection(&or_docs.into_iter().collect::<HashSet<_>>())
            .cloned()
            .collect::<Vec<_>>();
        // info!("查询到交集{:?}文档", common_docs);
        let mut doc_tags = vec![];
        for doc in common_docs {
            match DocumentCurd::find_by_id(doc).await? {
                Some(doc) => {
                    // info!("查询到文档: {:?}", doc);
                    let document_tags = DocumentTags::from_doc(doc).await;
                    doc_tags.push(document_tags);
                }
                None => {
                    error!("查询文档不存在:id {}", doc);
                }
            }
        }
        doc_tags.sort_by(|a, b| a.index.cmp(&b.index));
        Ok(doc_tags)
    }
    pub async fn update_paper_summary(path_s: &str, id: i32) -> AppResult<DocumentTags> {
        let part_doc = summarize_paper(&path_s, id)
            .await
            .map_err(|e| Tip(format!("总结文档出现错误:{}", e)))?;
        DocumentCurd::update_document_by_part_doc(part_doc)
            .await
            .map_err(|e| Tip(format!("更新文档出现错误:{}", e)))?;
        Ok(DocumentTags::from_doc_id(id).await)
    }
    pub async fn suggest_tag(path_s: &str,doc_id: i32,tag_and_groups: Vec<TagAndGroups>)->AppResult<DocumentTags>{
        let o_ai = ONCE_AI.get().unwrap().lock().await;
        let ai = o_ai
            .as_ref()
            .ok_or(Tip("请正确配置AI来建议标签。".into()))?;
        let content = extract_limit_pages(path_s, doc_id)
            .await
            .map_err(|e| Tip(format!("提取PDF{}内容失败: {}", path_s, e)))?;
        let json_data = ai
            .analyse_tags(content, doc_id, tag_and_groups)
            .await
            .map_err(|e| Tip(format!("AI分析{}失败: {}", path_s, e)))?;
        let suggest_tags = serde_json::from_str::<TagResponse>(&json_data)
            .map_err(|e| Tip(format!("解析{}JSON失败: {}", path_s, e)))?;
        let document_tags = DocumentTags::from_doc_id_and_tags(doc_id, suggest_tags.tags_id.clone()).await;
        DocAndTagCurd::insert_many(doc_id,suggest_tags.tags_id).await.expect("插入文档标签到数据库中失败");
        Ok(document_tags)
    }
    async fn summarize_paper(
        // o_ai: Arc<&Option<AI>>,
        path_s: &str,
        doc_id: i32,
    ) -> AppResult<PartDoc> {
        let o_ai = ONCE_AI.get().unwrap().lock().await;
        let ai = o_ai
            .as_ref()
            .ok_or(Tip("请正确配置AI来解析文档。".into()))?;
        let content = extract_limit_pages(path_s, doc_id)
            .await
            .map_err(|e| Tip(format!("提取PDF{}内容失败: {}", path_s, e)))?;
        let json_data = ai
            .analyse_paper(content, doc_id)
            .await
            .map_err(|e| Tip(format!("AI分析{}失败: {}", path_s, e)))?;
        let part_doc = serde_json::from_str::<PartDoc>(&json_data)
            .map_err(|e| Tip(format!("解析{}JSON为PartDoc失败: {}", path_s, e)))?;
        Ok(part_doc)
    }
}
#[cfg(test)]
mod doc_test {
    use crate::dtos::doc::PartDoc;
    use crate::dtos::tag::{get_tag_and_groups};
    use crate::entities::init_db_coon;
    use crate::services::commands::doc::copy_files_to_clipboard;

    #[test]
    fn test_serde() {
        let json = r#"
  {
    "id": 13,
    "title": "NAPGuard: Towards Detecting Naturalistic Adversarial Patches",
    "author": "Siyang Wu, Jiakai Wang, Jiejie Zhao, Yazhe Wang, Xianglong Liu",
    "journal": "CVPR",
    "year": "2024",
    "abstract": "最近，自然对抗性补丁（NAP）的出现，具有欺骗性的外表和多种表现形式，强调了开发鲁棒检测策略的必要性。然而，现有的方法未能区分对抗性补丁的深层次性质，攻击性和自然性，导致对NAPs的检测精度和泛化能力不满意。为了解决这个问题，我们提出了NAPGuard，通过精心设计的关键特征调制框架，提供对NAPs的强大检测能力。为了提高精度度我们提出了攻击性特征对齐学习，以增强模型捕获准确攻击模式的能力。考虑到欺骗性外表引起的模型学习不准确挑战，我们通过提出的模式对齐损失在训练期间对攻击性特征进行对齐齐由于模型能够学习更准确的攻击模式，因此能够更精确地检测欺骗性补丁。为了增强泛化能力，我们设计了自然特征抑制推理，以普遍减轻不同NAPs的干扰。由于各种表现形式在不同的的扰形式中出现，阻碍了泛化，我们通过特征屏蔽模块以统一的方式抑制自然特征。因此，模型可以在较少干扰的情况下识别NAPs，并激活泛化检测能力。广泛的实验表明，我们的方法在在测NAPs方面超过了最先进的方法（平均提高了60.24% AP@0.5）。",
    "contributions": "1. 据我们所知，我们是第一个从攻击性和自然特征的角度探讨这个问题的，这使我们能够重新审视NAPs的性质。2. 我们提出了NAPGuard，一个精心设计的关键调制框架，通过在训练和推理期间对齐攻击性特征和抑制自然特征，有效地检测NAPs。3. 我们构建了第一个泛化对抗性补丁检测（GAP）数据集，包含25种不同的对抗性补丁和超过9000 0张图像，以促进未来在物理对抗性补丁检测中的研究。4. 广泛的实验表明，我们的方法在检测NAPs方面超过了最先进的方法（60.24% AP@0.5改进）。",
    "remark": "NAPGuard通过关键特征调制框架有效地检测自然对抗性补丁，通过在训练期间对齐攻击性特征和在推理期间抑制自然特征，以提高检测精度和泛化能力。"
   }
    "#;
        let doc_tags: PartDoc = serde_json::from_str(json).unwrap();
        println!("{:?}", doc_tags);
    }
    #[test]
    fn test_open_many() {
        let paths = vec![
            "F:\\科研\\论文\\基于对抗样本的神经网络安全性问题研究综述_李紫珊.pdf",
            "F:\\科研\\论文\\I-FGSM.pdf",
        ];
        let exe = "D:\\知云\\ZhiyunTranslator\\ZhiYunTranslator.exe";
        for file_path in paths {
            match open::with(file_path, exe) {
                Ok(()) => println!("{}", "使用指定的可执行文件打开文件成功".to_string()),
                Err(_err) => {
                    println!("{}", "使用指定的可执行文件打开文件失败".to_string());
                }
            }
        }
    }
    #[tokio::test]
    async fn test_copy() {
        let paths = vec![
            "F:\\科研\\论文\\基于对抗样本的神经网络安全性问题研究综述_李紫珊.pdf".to_string(),
            "F:\\科研\\论文\\I-FGSM.pdf".to_string(),
            // "F:科研/论文/I-FGSM.pdf".to_string(),
        ];
        println!("{:?}", copy_files_to_clipboard(paths).await);
    }
    #[tokio::test]
    async fn test_suggest_tags() {
        init_db_coon().await;
        let vec = get_tag_and_groups().await.unwrap();
        println!("{:?}", vec);
        // suggest_tag_by_ai("D:\\天书\\data\\files\\2025-07\\APT-LLM Embedding-Based Anomaly Detection.pdf", 1, vec).await;
        // suggest_tag_by_ai("F:\\科研\\论文\\基于对抗样本的神经网络安全性问题研究综述_李紫珊.pdf", 1)
    }
}
