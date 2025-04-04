use crate::entities::DB;
use crate::entities::doc_and_tag::DocToTag;
use crate::entities::prelude::{Document, Documents, Tag};
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};

/// 文档和及其对应的标签的组合，注意，这个s并不是DocumentTag的复数，而是一个Document和多个Tag的意思
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTags {
    pub index: i32,
    pub id: i32,
    pub title: String,
    pub file_name: String,
    pub author: Option<String>,
    pub r#abstract: Option<String>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub contributions: Option<String>,
    pub remark: Option<String>,
    pub path: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartDoc {
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub r#abstract: Option<String>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub contributions: Option<String>,
    pub remark: Option<String>,
}

impl DocumentTags {
    pub async fn from_doc(document: Document) -> Self {
        let db = DB.get().unwrap();
        let tags = document
            .find_linked(DocToTag)
            .all(db)
            .await
            .expect("查询文档标签失败");
        Self {
            index: document.index,
            id: document.id,
            title: document.title,
            file_name: document.file_name,
            author: document.author,
            r#abstract: document.r#abstract,
            year: document.year,
            journal: document.journal,
            contributions: document.contributions,
            remark: document.remark,
            path: document.path,
            tags,
        }
    }
    pub async fn from_doc_id(document_id: i32) -> Self {
        let db = DB.get().unwrap();
        let document = Documents::find_by_id(document_id)
            .one(db)
            .await
            .expect("查询文档失败")
            .expect("文档不存在");
        Self::from_doc(document).await
    }
}
