use log::{error, info};
use crate::app_errors::AppResult;
use crate::dtos::tag_card::{get_tag_and_groups, TagAndGroups};
use crate::entities::prelude::{Tag, TagGroup};
use crate::services::curd::tag::TagCurd;
use crate::services::curd::tag_group::TagGroupCurd;

#[tauri::command]
pub async fn query_tag_groups() -> Result<Vec<TagAndGroups>, String> {
    match get_tag_and_groups().await{
        Ok(data) => Ok(data),
        Err(e) => {
            let err_msg = format!("查询标签失败:{:?}", e);
            error!("{}", err_msg);
            Err(err_msg)
        },
    }
}
/// 插入标签（传入的index和id是无效值，将会被覆盖）
/// 由于标签的索引和id是自动生成的，所以会返回一个完整有效的Tag结构体。
#[tauri::command]
pub async fn create_tag(tag:Tag) -> Result<Tag, String> {
    match TagCurd::insert(tag).await{
        Ok(tag) => Ok(tag),
        Err(e) => {
            let err_msg = format!("插入标签失败:{:?}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
/// 插入标签组（传入的index和id是无效值，将会被覆盖）
/// 由于标签组的索引和id是自动生成的，所以会返回一个完整有效的TagGroup结构体。
#[tauri::command]
pub async fn create_tag_group(group_name:String)->Result<TagGroup,String>{
    match TagGroupCurd::insert(&group_name).await{
        Ok(group) => Ok(group),
        Err(e) => {
            let err_msg = format!("创建标签组{group_name}失败：{:?}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
#[tauri::command]
pub async fn update_tag_group_name(group_id:i32, group_name:String)->Result<(),String>{
    match TagGroupCurd::update_name(group_id, &group_name).await{
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = format!("修改标签组名{group_name}失败：{:?}", e);
            error!("{}", err_msg);
            Err(err_msg)
        }
    }
}


