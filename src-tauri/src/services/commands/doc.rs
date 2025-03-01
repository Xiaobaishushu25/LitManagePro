use crate::config::Config;
use crate::dtos::doc_tags::DocumentTags;
use crate::services::ai::AI;
use crate::services::commands::doc::doc_util::{handle_many_paths, handle_query_docs_by_tags};
use crate::services::curd::doc_and_tag::DocAndTagCurd;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Emitter, State};

#[tauri::command]
// pub async fn insert_docs(config: State<'_, Mutex<Config>>, o_ai:State<'_, Mutex<Option<AI>>>,app_handle: tauri::AppHandle,paths:Vec<String>,tags_id:Vec<i32>) -> Result<(), String> {
pub async fn insert_docs(
    config: State<'_, Mutex<Config>>,
    o_ai: State<'_, tokio::sync::Mutex<Option<AI>>>,
    app_handle: tauri::AppHandle,
    paths: Vec<String>,
    tags_id: Vec<i32>,
) -> Result<(), String> {
    let flag = { config.lock().unwrap().ai_config.use_ai };
    let o_ai = o_ai.lock().await;
    match handle_many_paths(flag, &o_ai, app_handle, paths, tags_id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("插入文档失败：{:?}", e);
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
            error!("查找文档失败：{:?}", e);
            Err("查找文档失败".to_string())
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartDoc {
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub r#abstract: Option<String>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub contributions: Option<String>,
    pub remark: Option<String>,
}
#[allow(dead_code)]
mod doc_util {
    use crate::app_errors::AppResult;
    use crate::dtos::doc_tags::DocumentTags;
    use crate::entities::prelude::Document;
    use crate::services::ai::AI;
    use crate::services::commands::doc::PartDoc;
    use crate::services::curd::doc_and_tag::DocAndTagCurd;
    use crate::services::curd::document::DocumentCurd;
    use crate::services::util::pdf::extract_limit_pages;
    use log::{error, info};
    use std::collections::HashSet;
    use std::path::Path;
    use std::sync::Mutex;
    use tauri::{Emitter, State};
    use tracing::instrument;

    pub(crate) async fn handle_many_paths(
        use_ai: bool,
        o_ai: &Option<AI>,
        app_handle: tauri::AppHandle,
        paths: Vec<String>,
        tags_id: Vec<i32>,
    ) -> AppResult<()> {
        // pub(crate) async fn handle_many_paths(use_ai:bool, o_ai:State<'_, Mutex<Option<AI>>>, app_handle: tauri::AppHandle,paths: Vec<String>,tags_id:Vec<i32>) -> AppResult<()> {
        for path_s in paths {
            //根据path获取文件信息
            // Convert the string path to a Path
            let path = Path::new(&path_s);
            if !path.exists() {
                //虽然感觉不可能不存在的情况
                let err_msg = format!("路径不存在: {}", path.display());
                error!("{}", err_msg);
                let _ = app_handle.emit("backend_message", err_msg);
                continue;
            }
            if path.is_dir() {
                let err_msg = format!("不支持解析文件夹: {}", path.display());
                error!("{}", err_msg);
                let _ = app_handle.emit("backend_message", err_msg);
                continue;
            }
            // path.file_stem()
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    info!("File name: {}", file_name_str);
                    // let _ = app_handle.emit("backend_message", format!("正在处理文件: {}", file_name_str));
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
                                if ext.to_str().unwrap() == "pdf" {
                                    //todo 处理PDF文件,提取作者名、标题、摘要等信息
                                    if use_ai {
                                        // let o_ai = o_ai.lock().unwrap();
                                        match o_ai {
                                            Some(ai) => {
                                                match extract_limit_pages(&path_s, id).await {
                                                    Ok(content) => {
                                                        match ai.analyse_paper(content, id).await {
                                                            Ok(string) => {
                                                                info!("AI分析结果{}", string);
                                                                match serde_json::from_str::<PartDoc>(
                                                                    &string,
                                                                ) {
                                                                    Ok(part_doc) => {
                                                                        info!(
                                                                            "AI分析成功: {:?}",
                                                                            part_doc
                                                                        );
                                                                        if let Err(e) = DocumentCurd::update_document(part_doc).await{
                                                                            error!("更新文档失败: {:?}", e);
                                                                            let _ = app_handle.emit("backend_message", format!("更新文档失败: {:?}", e));
                                                                        }else {
                                                                            let document_tags = DocumentTags::from_doc_id(id).await;
                                                                            let _ = app_handle.emit("doc_update", document_tags);
                                                                        }
                                                                        // let _ = app_handle.emit("insert_doc", document_tags);
                                                                    }
                                                                    Err(e) => {
                                                                        let err_msg = format!(
                                                                            "AI分析失败: {:?}",
                                                                            e
                                                                        );
                                                                        error!("{}", err_msg);
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                let err_msg =
                                                                    format!("AI分析失败: {:?}", e);
                                                                error!("{}", err_msg);
                                                                let _ = app_handle.emit(
                                                                    "backend_message",
                                                                    err_msg,
                                                                );
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        let err_msg =
                                                            format!("提取PDF内容失败: {:?}", e);
                                                        error!("{}", err_msg);
                                                        let _ = app_handle
                                                            .emit("backend_message", err_msg);
                                                    }
                                                }
                                            }
                                            None => {
                                                let err_msg =
                                                    "请正确配置AI来解析文档。".to_string();
                                                error!("{}", err_msg);
                                                let _ = app_handle.emit("backend_message", err_msg);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let err_msg = format!("插入文档{}失败: {:?}", path.display(), e);
                            error!("{}", err_msg);
                            let _ = app_handle.emit("backend_message", err_msg);
                        }
                    }
                } else {
                    let err_msg = format!("无效的文件名: {}", path.display());
                    error!("{}", err_msg);
                    let _ = app_handle.emit("backend_message", err_msg);
                }
            } else {
                //文件夹的情况
                let err_msg = format!("未获取到文件名: {}", path.display());
                error!("{}", err_msg);
                let _ = app_handle.emit("backend_message", err_msg);
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
        Ok(doc_tags)
    }
}
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
