use sea_orm::{Linked, ModelTrait};
use serde::{Deserialize, Serialize};
use crate::entities::DB;
use crate::entities::doc_and_tag::DocToTag;
use crate::entities::prelude::{Document, Tag};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct DocumentTags{
    pub index: i32,
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub r#abstract: Option<String>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub contributions: Option<String>,
    pub remark: Option<String>,
    pub path: String,
    pub tags:Vec<Tag>
}
impl DocumentTags {
    pub async fn from_doc(document: Document)->Self{
        let db = DB.get().unwrap();
        let tags = document.find_linked(DocToTag).all(db).await.expect("查询文档标签失败");
        Self{
            index:document.index,
            id:document.id,
            title:document.title,
            author:document.author,
            r#abstract:document.r#abstract,
            year:document.year,
            journal:document.journal,
            contributions:document.contributions,
            remark:document.remark,
            path:document.path,
            tags,
        }
    }
}