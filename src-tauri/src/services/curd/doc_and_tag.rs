use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::doc_and_tag::Column;
use crate::entities::prelude::{DocAndTag, DocAndTags, Document, Documents};
use crate::entities::{document, init_db_coon};
use sea_orm::QueryFilter;
use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QuerySelect};
use tracing::instrument;

pub struct DocAndTagCurd;
impl DocAndTagCurd {
    pub async fn insert(doc_id: i32, tag_id: i32) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let model = DocAndTag { tag_id, doc_id };
        DocAndTags::insert(model.into_active_model())
            .exec(db)
            .await?;
        Ok(())
    }
    pub async fn insert_many(doc_id: i32, tag_ids: Vec<i32>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let models = tag_ids
            .iter()
            .map(|tag_id| {
                let model = DocAndTag {
                    tag_id: *tag_id,
                    doc_id,
                };
                model.into_active_model()
            })
            .collect::<Vec<_>>();
        // DocAndTags::insert(model.into_active_model()).exec(db).await?;
        DocAndTags::insert_many(models).exec(db).await?;
        Ok(())
    }
    pub async fn update_many(doc_id: i32, tag_ids: Vec<i32>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        //查询所有doc_id等于doc_id的记录，并删除
        DocAndTags::delete_many()
            .filter(Column::DocId.eq(doc_id))
            .exec(db)
            .await?;
        Self::insert_many(doc_id, tag_ids).await?;
        Ok(())
    }
    #[instrument]
    pub async fn delete(doc_id: i32, tag_id: i32) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let model = DocAndTag { tag_id, doc_id };
        DocAndTags::delete(model.into_active_model())
            .exec(db)
            .await?;
        Ok(())
    }
    /// 根据标签 ID组 查询文档。
    ///
    /// # 功能描述
    /// 根据给定的标签 ID 列表，查找与这些标签相关联的文档 ID。如果 `and` 参数为 `true`，
    /// 则返回与 **所有** 标签相关联的文档；如果为 `false`，返回与 **至少一个** 标签相关联的文档。
    /// 如果没有提供标签 ID，返回所有文档的 doc_id。
    ///
    /// # 参数
    /// - `tag_ids`: 标签 ID 列表，用于筛选相关文档。
    /// - `and`: 布尔值，决定查询模式：
    ///   - 如果为 `true`，查找与所有 `tag_ids` 相关联的文档。
    ///   - 如果为 `false`，查找与至少一个 `tag_ids` 相关联的文档。
    ///
    /// # 返回
    /// 成功时返回一个包含文档 ID 的向量。如果查询失败，返回一个 `AppError`。
    ///
    /// # 示例
    /// - 查找包含标签 `1` 和 `2` 相关联的文档：
    ///   ```rust
    ///   let tags = vec![1, 2];
    ///   match find_documents_with_tags(tags, true).await {
    ///       Ok(docs) => println!("与标签 1 和 2 都相关联的文档：{:?}", docs),
    ///       Err(err) => eprintln!("Error: {}", err),
    ///   }
    ///   ```
    ///
    /// - 查找与标签 `3` 或 `4` 相关联的文档：
    ///   ```rust
    ///   let tags = vec![3, 4];
    ///   match find_documents_with_tags(tags, false).await {
    ///       Ok(docs) => println!("与标签 3 或 4 相关联的文档：{:?}", docs),
    ///       Err(err) => eprintln!("Error: {}", err),
    ///   }
    ///   ```
    pub async fn find_documents_with_tags(tag_ids: Vec<i32>, and: bool) -> AppResult<Vec<i32>> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let doc_ids = if tag_ids.is_empty() {
            // 如果 tag_ids 为空，返回所有文档的 doc_id
            Documents::find()
                .select_only()
                .column(document::Column::Id)
                .distinct() // 避免重复的 doc_id
                .into_tuple::<i32>()
                .all(db)
                .await?
        } else {
            let len = tag_ids.len() as u64;
            let select = DocAndTags::find()
                .select_only()
                .columns([Column::DocId, Column::TagId])
                .filter(Column::TagId.is_in(tag_ids))
                .group_by(Column::DocId);
            // 根据 and 参数决定是否添加 HAVING 条件
            let select = if and {
                select.having(Expr::col(Column::DocId).count().eq(len)) //对查询结果按 doc_id 进行分组。过滤出那些分组后的 doc_id 的记录数等于 len 的分组。
            } else {
                select
            };
            select
                .all(db)
                .await?
                .into_iter()
                .map(|x| x.doc_id)
                .collect::<Vec<_>>()
        };
        Ok(doc_ids)
    }
}
#[tokio::test]
async fn test_find_documents_with_all_tags() {
    init_db_coon().await;
    let tags = DocAndTagCurd::find_documents_with_tags(vec![], false)
        .await
        .unwrap();
    for i in tags {
        println!("{}", i);
    }
}
