use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, ModelTrait, NotSet};
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::prelude::{ActiveTagGroup, TagGroup, TagGroups};

pub struct TagGroupCurd;
impl TagGroupCurd {
    pub async fn insert(tag_group_name: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tag_group = ActiveTagGroup{
            id: NotSet,
            name: Set(tag_group_name),
        };
        TagGroups::insert(tag_group).exec(db).await?;
        Ok(())
    }
    pub async fn delete(tag_group_id: i32) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tag_group = TagGroups::find_by_id(tag_group_id).one(db).await?;
        if let Some(tag_group) = tag_group {
            tag_group.delete(db).await?;
        }
        Ok(())
    }
    pub async fn query_tag_groups() -> AppResult<Vec<TagGroup>> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tag_groups = TagGroups::find().all(db).await?;
        Ok(tag_groups)
    }
}