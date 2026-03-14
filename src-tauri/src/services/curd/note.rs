use crate::app_errors::{AppError, AppResult};
use crate::entities::prelude::*;
use sea_orm::{EntityTrait, QueryFilter, QueryOrder, ActiveModelTrait, ColumnTrait, ModelTrait};
use crate::app_errors::AppError::Tip;
use crate::entities::note::{Column, Model};

/// 笔记 CRUD 操作
pub struct NoteCurd;

impl NoteCurd {
    /// 创建笔记
    pub async fn create(note: ActiveNote) -> AppResult<Model> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let result = note.insert(db).await?;
        Ok(result)
    }

    /// 根据文档 ID 获取所有笔记
    pub async fn find_by_document_id(document_id: i32) -> AppResult<Vec<Model>> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let notes = Notes::find()
            .filter(Column::DocumentId.eq(document_id))
            .order_by_asc(Column::UpdatedAt)
            .all(db)
            .await?;
        Ok(notes)
    }

    /// 获取所有笔记
    pub async fn find_all() -> AppResult<Vec<Model>> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let notes = Notes::find()
            .order_by_desc(Column::UpdatedAt)
            .all(db)
            .await?;
        Ok(notes)
    }

    /// 根据 ID 获取笔记
    pub async fn find_by_id(id: i32) -> AppResult<Option<Model>> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let note = Notes::find_by_id(id).one(db).await?;
        Ok(note)
    }

    /// 更新笔记
    pub async fn update(note: ActiveNote) -> AppResult<Model> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let result = note.update(db).await?;
        Ok(result)
    }

    /// 删除笔记
    pub async fn delete(id: i32) -> AppResult<()> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let note = Notes::find_by_id(id).one(db).await?;
        if let Some(note) = note {
            note.delete(db).await?;
        }
        Ok(())
    }

    /// 根据文档id批量删除笔记
    pub async fn delete_by_document_id(document_id: i32) -> AppResult<u64> {
        let db = crate::entities::DB.get().ok_or(Tip("数据库未初始化".into()))?;
        let result = Notes::delete_many()
            .filter(Column::DocumentId.eq(document_id))
            .exec(db)
            .await?;
        Ok(result.rows_affected)
    }
}
