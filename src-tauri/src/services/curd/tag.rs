use sea_orm::{EntityTrait, IntoActiveModel, ModelTrait, NotSet};
use sea_orm::ActiveValue::Set;
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::prelude::{ActiveTag, Tag, Tags};

pub struct TagCurd;
impl TagCurd {
    pub async fn insert(tag_group_id: i32, tag_value: String, color: String) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let active_tag = ActiveTag{
            group_id: Set(tag_group_id),
            id: NotSet,
            value: Set(tag_value),
            color: Set(color),
        };
        Tags::insert(active_tag).exec(db).await?;
        Ok(())
    }
    // pub async fn insert(tag_group_id: i32, tag_value: String, color: String) -> AppResult<()> {
    pub async fn insert_many(tags:Vec<Tag>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let active_tags = tags.into_iter().map(|tag| {
            let mut active_model = tag.into_active_model();
            active_model.id = NotSet;
            active_model
        }).collect::<Vec<_>>();
        Tags::insert_many(active_tags).exec(db).await?;
        Ok(())
    }
    pub async fn delete(tag_id: i32) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tag = Tags::find_by_id(tag_id).one(db).await?;
        if let Some(tag) = tag {
            tag.delete(db).await?;
        }
        Ok(())
    }
    pub async fn query_tags() -> AppResult<Vec<Tag>> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tags = Tags::find().all(db).await?;
        Ok(tags)
    }
}