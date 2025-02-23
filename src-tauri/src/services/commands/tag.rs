use log::error;
use crate::dtos::tag_card::{get_tag_and_groups, TagAndGroups};


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


