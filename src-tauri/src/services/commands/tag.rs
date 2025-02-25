use log::{error, info};
use crate::app_errors::AppResult;
use crate::dtos::tag_card::{get_tag_and_groups, TagAndGroups};
use crate::entities::prelude::{Tag, TagGroup};
use crate::services::curd::tag::TagCurd;
use crate::services::curd::tag_group::TagGroupCurd;

#[tauri::command]
pub async fn query_tags() -> Result<Vec<TagAndGroups>, String> {
    match get_tag_and_groups().await{
        Ok(data) => Ok(data),
        Err(e) => {
            let err_msg = format!("查询标签失败:{:?}", e);
            error!("{}", err_msg);
            Err(err_msg)
        },
    }
}
#[tauri::command]
pub async fn insert_tag(tag:Tag) -> Result<i32, String> {
    match TagCurd::insert(tag).await{
        Ok(id) => Ok(id),
        Err(e) => {
            let err_msg = format!("插入标签失败:{:?}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
#[tauri::command]
pub async fn create_tag_group(group_name:String)->Result<i32,String>{
    match TagGroupCurd::insert(&group_name).await{
        Ok(id) => Ok(id),
        Err(e) => {
            let err_msg = format!("创建标签组{group_name}失败：{:?}", e);
            error!(err_msg);
            Err(err_msg)
        }
    }
}
#[tauri::command]
pub async fn update_tag_group_name(group_id:i32, group_name:String)->Result<(),String>{
    match TagGroupCurd::update(group_id, &group_name).await{
        Ok() => {Ok(())}
        Err(e) => {
            let err_msg = format!("修改标签组名{group_name}失败：{:?}", e);
            error!(err_msg);
            Err(err_msg)
        }
    }
}


