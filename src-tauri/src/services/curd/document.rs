use anyhow::Context;
use sea_orm::QueryFilter;
use crate::app_errors::AppError::Tip;
use crate::app_errors::AppResult;
use crate::entities::document::Column;
use crate::entities::prelude::{DocAndTags, Document, Documents};
use crate::services::commands::doc::PartDoc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, NotSet, QuerySelect};

pub struct DocumentCurd;
impl DocumentCurd {
    pub async fn insert(mut doc: Document) -> AppResult<Document> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let max_index = Documents::find()
            .select_only()
            .expr(Column::Index.max())
            .into_tuple::<Option<i32>>()
            .one(db)
            .await?
            .unwrap()
            .unwrap_or(0);
        let mut active_doc = doc.clone().into_active_model();
        active_doc.index = Set(max_index + 1);
        active_doc.id = NotSet;
        let insert_result = Documents::insert(active_doc).exec(db).await?;
        doc.index = max_index + 1;
        doc.id = insert_result.last_insert_id;
        Ok(doc)
    }
    //由于有index的问题，加上回调问题，所以暂时不提供批量插入
    // pub async fn insert_many(docs:Vec<Document>) -> AppResult<()> {
    //     let db = crate::entities::DB
    //         .get()
    //         .ok_or(Tip("数据库未初始化".into()))?;
    //     let active_tags = docs.into_iter().map(|doc| {
    //         let mut active_model = doc.into_active_model();
    //         active_model.id = NotSet;
    //         active_model
    //     }).collect::<Vec<_>>();
    //     Documents::insert_many(active_tags).exec(db).await?;
    //     Ok(())
    // }
    ///根据文档id删除文档，同时删除文档和标签的关联关系
    /// 应该用不到，而且实现还存在问题，需要先删除doc_and_tag表的关联关系，再删除文档
    // pub async fn delete(id: i32) -> AppResult<()> {
    //     let db = crate::entities::DB
    //         .get()
    //         .ok_or(Tip("数据库未初始化".into()))?;
    //     let doc = Documents::find_by_id(id).one(db).await?;
    //     if let Some(doc) = doc {
    //         let vec = doc.find_related(DocAndTags).all(db).await?;
    //         for doc_and_tag in vec {
    //             doc_and_tag.delete(db).await?;
    //         }
    //         doc.delete(db).await?;
    //     }
    //     Ok(())
    // }
    /// 批量删除文档，同时删除文档和标签的关联关系
    /// 由于有外键关系，所以需要先删除doc_and_tag表的关联关系，再删除文档
    pub async fn delete_many(ids: Vec<i32>) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        // Documents::delete_many().filter(Column::Id.is_in(ids.clone())).exec(db).await.with_context(|| "删除文档失败")?;
        DocAndTags::delete_many().filter(crate::entities::doc_and_tag::Column::DocId.is_in(ids.clone())).exec(db).await?;
        Documents::delete_many().filter(Column::Id.is_in(ids.clone())).exec(db).await.map_err(|e| Tip(format!("删除文档失败:{:#}",e)))?;
        Ok(())
    }
    pub async fn update_detail(doc_new: Document) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let doc = Documents::find_by_id(doc_new.id).one(db).await?;
        if let Some(doc) = doc {
            let mut doc = doc.into_active_model();
            doc.title = Set(doc_new.title);
            doc.r#abstract = Set(doc_new.r#abstract);
            doc.author = Set(doc_new.author);
            doc.year = Set(doc_new.year);
            doc.journal = Set(doc_new.journal);
            doc.contributions = Set(doc_new.contributions);
            doc.remark = Set(doc_new.remark);
            Documents::update(doc).exec(db).await?;
        }
        Ok(())
    }
    /// 根据部分文档更新文档,这个函数目前仅用于ai总结完成后更新文档信息
    pub async fn update_document_by_part_doc(part_doc: PartDoc) -> AppResult<()> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        let doc = Documents::find_by_id(part_doc.id).one(db).await?;
        if let Some(doc) = doc {
            let mut doc = doc.into_active_model();
            doc.r#abstract = Set(part_doc.r#abstract);
            doc.title = Set(part_doc.title);
            doc.author = Set(part_doc.author);
            doc.year = Set(part_doc.year);
            doc.journal = Set(part_doc.journal);
            doc.contributions = Set(part_doc.contributions);
            doc.remark = Set(part_doc.remark);
            Documents::update(doc).exec(db).await?;
        }
        Ok(())
    }
    pub async fn find_by_id(doc_id: i32) -> AppResult<Option<Document>> {
        let db = crate::entities::DB
            .get()
            .ok_or(Tip("数据库未初始化".into()))?;
        // Documents::find_by_id(doc_id).one(db).await?;
        Ok(Documents::find_by_id(doc_id).one(db).await?)
    }
}
