use sea_orm::entity::prelude::*;
use sea_orm::sqlx::types::chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    /// 关联的文档 ID
    pub document_id: i32,
    /// 笔记标题
    pub title: Option<String>,
    /// 笔记内容 (Markdown 格式)
    pub content: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl Model {
    pub fn new(document_id: i32, content: String) -> Self {
        Self {
            id: 0,//这里id实际是自增的，所以这里初始化为0
            document_id,
            title: None,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::document::Entity",
        from = "Column::DocumentId",
        to = "super::document::Column::Id"
    )]
    Document,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Document.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
