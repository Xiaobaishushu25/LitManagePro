use sea_orm::QueryFilter;
use log::error;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, NotSet, QuerySelect};
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::prelude::{ActiveTagGroup, TagGroup, TagGroups};
use crate::entities::tag_group::Column;

pub struct TagGroupCurd;
impl TagGroupCurd {
    pub async fn insert(tag_group_name: &str) -> AppResult<TagGroup> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        //查询当前组的index记录的最大值
        let max_index = TagGroups::find()
            .select_only()
            .expr(Column::Index.max())
            .into_tuple::<Option<i32>>()
            .one(db)
            .await?
            .unwrap()
            .unwrap_or(0);
        let tag_group = ActiveTagGroup{
            index: Set(max_index + 1),
            id: NotSet,
            name: Set(tag_group_name.into()),
        };
        let insert_result = TagGroups::insert(tag_group).exec(db).await?;
        Ok(TagGroup{
            index: max_index+1,
            id: insert_result.last_insert_id,
            name: tag_group_name.into(),
        })
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
    pub async fn update_name(group_id:i32, new_name:&str) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tag_group = TagGroups::find_by_id(group_id).one(db).await?;
        if let Some(tag_group) = tag_group {
            let mut active_model = tag_group.into_active_model();
            active_model.name=Set(new_name.to_string());
            TagGroups::insert(active_model).exec(db).await?;
        }else {
            //这里正常来说不会出错，也没必要panic，记录一下就行了
            error!("未找到group id为{group_id}的标签组");
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