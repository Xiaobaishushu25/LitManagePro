use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "document")]
pub struct Model {
    pub index: i32,
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub r#abstract: Option<String>,
    pub year: Option<String>,
    pub journal: Option<String>,
    pub contributions: Option<String>,
    pub remark: Option<String>,
    pub path: String,
}
impl Model {
    pub fn new(
        title: String,
        path: String,
    ) -> Self {
        Self {
            index:0,
            id: 0,
            title,
            author:None,
            r#abstract:None,
            year:None,
            journal:None,
            contributions:None,
            remark:None,
            path,
        }
    }
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::doc_and_tag::Entity")]
    DocAndTag,
}

impl Related<super::doc_and_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DocAndTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}