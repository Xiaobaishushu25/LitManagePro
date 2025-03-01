use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::prelude::{DocAndTag, DocAndTags, Tag, Tags};
use crate::entities::tag::Column;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, NotSet, QuerySelect};

pub struct TagCurd;
impl TagCurd {
    ///参数中的tag的index字段和id字段是自动生成的，所以需要重新设置。
    /// 异步插入标签，并返回插入后的完整Tag（有索引和id数据）。
    ///
    /// 此函数首先检查数据库是否已初始化，然后找到当前最大的索引值，
    /// 并将新标签的索引值设置为最大索引值加一，以确保标签的顺序。
    /// 最后，将标签插入数据库，并返回插入标签。
    ///
    /// # 参数
    ///
    /// * `tag` - 要插入的标签数据模型。
    ///
    /// # 返回值
    ///
    /// * `AppResult<i32>` - 插入成功后返回一个包含标签结构体的结果类型。
    ///
    /// # 错误处理
    ///
    /// * 如果数据库未初始化，将返回一个自定义错误提示。
    /// * 如果数据库查询或插入操作失败，将传播相应的错误。
    pub async fn insert(tag: Tag) -> AppResult<Tag> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let max_index = Tags::find()
            .select_only()
            .expr(Column::Index.max())
            .into_tuple::<Option<i32>>()
            .one(db)
            .await?
            .unwrap()
            .unwrap_or(0);
        let mut active_tag = tag.clone().into_active_model();
        active_tag.index = Set(max_index + 1);
        active_tag.id = NotSet;
        let insert_result = Tags::insert(active_tag).exec(db).await?;
        Ok(Tag {
            index: max_index + 1,
            id: insert_result.last_insert_id,
            ..tag
        })
    }
    /// 删除标签，根据给定的标签ID。
    /// 需要同时删除该标签和文档之间的关联关系。
    pub async fn delete(tag_id: i32) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let tag = Tags::find_by_id(tag_id).one(db).await?;
        if let Some(tag) = tag {
            let vec = tag.find_related(DocAndTags).all(db).await?;
            for doc_and_tag in vec {
                doc_and_tag.delete(db).await?;
            }
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
