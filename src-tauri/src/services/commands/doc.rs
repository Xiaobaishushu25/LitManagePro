use log::{error, info};
use crate::dtos::doc_tags::DocumentTags;
use crate::services::commands::doc::doc_util::{handle_many_paths, handle_query_docs_by_tags};

#[tauri::command]
pub async fn insert_docs(app_handle: tauri::AppHandle,paths:Vec<String>,tags_id:Vec<i32>) -> Result<(), String> {
    match handle_many_paths(app_handle,paths,tags_id).await{
        Ok(_) => Ok(()),
        Err(e) => {
            error!("插入文档失败：{:?}", e);
            Err("插入文档失败".to_string())
        },
    }
}
#[tauri::command]
pub async fn query_docs_by_tags(and_tags_id:Vec<i32>,or_tags_id:Vec<i32>) -> Result<Vec<DocumentTags>, String> {
    match handle_query_docs_by_tags(and_tags_id,or_tags_id).await{
        Ok(data) => Ok(data),
        Err(e) => {
            error!("查找文档失败：{:?}", e);
            Err("查找文档失败".to_string())
        },
    }
}
#[allow(dead_code)]
mod doc_util {
    use std::collections::HashSet;
    use std::path::Path;
    use log::{error, info};
    use tauri::Emitter;
    use tracing::instrument;
    use crate::app_errors::AppResult;
    use crate::dtos::doc_tags::DocumentTags;
    use crate::entities::prelude::Document;
    use crate::services::curd::doc_and_tag::DocAndTagCurd;
    use crate::services::curd::document::DocumentCurd;

    pub(crate) async  fn handle_many_paths(app_handle: tauri::AppHandle,paths: Vec<String>,tags_id:Vec<i32>) -> AppResult<()> {
        for path_s in paths {
            //根据path获取文件信息
            // Convert the string path to a Path
            let path = Path::new(&path_s);
            if !path.exists(){//虽然感觉不可能不存在的情况
                let err_msg = format!("路径不存在: {}", path.display());
                error!("{}", err_msg);
                let _ = app_handle.emit("backend_message", err_msg);
                continue;
            }
            if path.is_dir(){
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
                    match DocumentCurd::insert(Document::new(file_name_str.into(), path_s.clone())).await{
                        Ok(doc) => {
                            if !tags_id.is_empty() {
                                DocAndTagCurd::insert_many(doc.id,tags_id.clone()).await.expect("插入文档和标签关系失败");
                            }
                            let document_tags = DocumentTags::from_doc(doc).await;
                            info!("插入文档成功: {:?}", document_tags);
                            let _ = app_handle.emit("insert_doc", document_tags);
                        },
                        Err(e) => {
                            let err_msg = format!("插入文档{}失败: {:?}", path.display(),e);
                            error!("{}", err_msg);
                            let _ = app_handle.emit("backend_message", err_msg);
                        },
                    }
                    if let Some(ext) = path.extension(){
                        if ext.to_str().unwrap() == "pdf"{
                            //todo 处理PDF文件,提取作者名、标题、摘要等信息
                        }
                    }
                } else {
                    let err_msg = format!("无效的文件名: {}", path.display());
                    error!("{}", err_msg);
                    let _ = app_handle.emit("backend_message", err_msg);
                }
            } else {//文件夹的情况
                let err_msg = format!("未获取到文件名: {}", path.display());
                error!("{}", err_msg);
                let _ = app_handle.emit("backend_message", err_msg);
            }
            
        }
        Ok(())
    }
    #[instrument]
    pub async fn handle_query_docs_by_tags(and_tags_id:Vec<i32>,or_tags_id:Vec<i32>)->AppResult<Vec<DocumentTags>>{
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
            match DocumentCurd::find_by_id(doc).await?{
                Some(doc) => {
                    // info!("查询到文档: {:?}", doc);
                    let document_tags = DocumentTags::from_doc(doc).await;
                    doc_tags.push(document_tags);
                },
                None => {
                    error!("查询文档不存在:id {}", doc);
                }
            }
        }
        Ok(doc_tags)
    }
}