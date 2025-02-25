use log::info;
use sea_orm::{ModelTrait, Related};
use serde::{Deserialize, Serialize};
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::prelude::{Tag, TagGroup, TagGroups, Tags};
use crate::services::curd::tag_group::TagGroupCurd;
#[derive(Debug, Clone, Serialize)]
pub struct TagAndGroups {
    tag_group: TagGroup,
    tags: Vec<Tag>
}
pub async fn get_tag_and_groups() -> AppResult<Vec<TagAndGroups>> {
    let db = crate::entities::DB
        .get()
        .ok_or(Tip("数据库未初始化".into()))?;
    let tag_groups = TagGroupCurd::query_tag_groups().await?;
    let mut result: Vec<TagAndGroups> = Vec::new();
    for tag_group in tag_groups {
        let tags = tag_group.find_related(Tags).all(db).await?;
        result.push(TagAndGroups {
            tag_group,
            tags,
        });
    }
    Ok(result)
}