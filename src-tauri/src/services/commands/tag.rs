use crate::dtos::doc::DocumentTags;
use crate::dtos::tag::{TagAndGroups, get_tag_and_groups};
use crate::entities::prelude::{Tag, TagGroup};
use crate::services::curd::doc_and_tag::DocAndTagCurd;
use crate::services::curd::tag::TagCurd;
use crate::services::curd::tag_group::TagGroupCurd;
use log::error;
use tauri::Emitter;

#[tauri::command]
pub async fn query_tag_groups() -> Result<Vec<TagAndGroups>, String> {
    match get_tag_and_groups().await {
        Ok(data) => Ok(data),
        Err(e) => {
            let err_msg = format!("查询标签失败:{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
/// 插入标签（传入的index和id是无效值，将会被覆盖）
/// 由于标签的索引和id是自动生成的，所以会返回一个完整有效的Tag结构体。
#[tauri::command]
pub async fn create_tag(tag: Tag) -> Result<Tag, String> {
    match TagCurd::insert(tag).await {
        Ok(tag) => Ok(tag),
        Err(e) => {
            let err_msg = format!("插入标签失败:{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
/// 插入标签（传入的index和id是无效值，将会被覆盖）
/// 由于标签的索引和id是自动生成的，所以会返回一个完整有效的Tag结构体。
#[tauri::command]
pub async fn delete_tag(id: i32) -> Result<(), String> {
    match TagCurd::delete(id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = format!("删除标签失败:{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
/// 插入标签组（传入的index和id是无效值，将会被覆盖）
/// 由于标签组的索引和id是自动生成的，所以会返回一个完整有效的TagGroup结构体。
#[tauri::command]
pub async fn create_tag_group(group_name: String) -> Result<TagGroup, String> {
    match TagGroupCurd::insert(&group_name).await {
        Ok(group) => Ok(group),
        Err(e) => {
            let err_msg = format!("创建标签组{group_name}失败：{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
#[tauri::command]
pub async fn delete_group(id: i32) -> Result<(), String> {
    match TagGroupCurd::delete(id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = format!("删除标签组失败：{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
#[tauri::command]
pub async fn rename_tag_group(id: i32, name: String) -> Result<(), String> {
    match TagGroupCurd::update_name(id, &name).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = format!("修改标签组名为{name}失败：{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
//todo 这个函数好像没用到
#[tauri::command]
pub async fn insert_doc_tag(doc_id: i32, tag_id: i32) -> Result<(), String> {
    match DocAndTagCurd::insert(doc_id, tag_id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = format!("保存文档标签失败：{}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
#[tauri::command]
pub async fn delete_doc_tag(
    app_handle: tauri::AppHandle,
    doc_id: i32,
    tag_id: i32,
) -> Result<(), String> {
    match DocAndTagCurd::delete(doc_id, tag_id).await {
        Ok(_) => {
            let document_tags = DocumentTags::from_doc_id(doc_id).await;
            let _ = app_handle.emit("doc_update", document_tags);
            Ok(())
        }
        Err(e) => {
            error!("删除文档的标签失败：{}", e);
            Err("删除文档的标签失败".to_string())
        }
    }
}
/// 更新标签组的索引
#[tauri::command]
pub async fn reindex_tag_group(
    tag_groups: Vec<TagGroup>,
) -> Result<(), String> {
    match TagGroupCurd::update_index(tag_groups).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("更新标签组的索引失败：{}", e);
            Err("更新标签组的索引失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn update_doc_tags(
    app_handle: tauri::AppHandle,
    doc_id: i32,
    tag_ids: Vec<i32>,
) -> Result<(), String> {
    match DocAndTagCurd::update_many(doc_id, tag_ids).await {
        Ok(_) => {
            let document_tags = DocumentTags::from_doc_id(doc_id).await;
            let _ = app_handle.emit("doc_update", document_tags);
            Ok(())
        }
        Err(e) => {
            error!("更新文档的标签失败：{}", e);
            Err("更新文档的标签失败".to_string())
        }
    }
}
