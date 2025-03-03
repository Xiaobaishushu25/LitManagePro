use crate::entities::doc_and_tag;
use crate::entities::prelude::{Documents, Tags};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "doc_and_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tag_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub doc_id: i32,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::document::Entity",
        from = "Column::DocId",
        to = "super::document::Column::Id"
    )]
    Document,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}
impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Document.def()
    }
}
impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}

pub struct DocToTag;
impl Linked for DocToTag {
    type FromEntity = Documents;
    type ToEntity = Tags;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            doc_and_tag::Relation::Document.def().rev(),
            doc_and_tag::Relation::Tag.def(),
        ]
    }
}
pub struct TagToDoc;
impl Linked for TagToDoc {
    type FromEntity = Tags;
    type ToEntity = Documents;
    fn link(&self) -> Vec<RelationDef> {
        vec![
            doc_and_tag::Relation::Tag.def().rev(),
            doc_and_tag::Relation::Document.def(),
        ]
    }
}

#[cfg(test)]
mod test {
    use crate::entities::doc_and_tag::{DocToTag, TagToDoc};
    use crate::entities::prelude::{Documents, Tags};
    use crate::entities::{DB, init_db_coon};
    use sea_orm::{EntityTrait, ModelTrait};

    #[tokio::test]
    async fn test_relation() {
        init_db_coon().await;
        let db = DB.get().unwrap();
        let doc = Documents::find_by_id(1).one(db).await.unwrap().unwrap();
        let select = doc.find_linked(DocToTag).all(db).await.unwrap();
        for tag in select {
            println!("{:?}", tag);
        }
        let doc = Tags::find_by_id(33).one(db).await.unwrap().unwrap();
        let select = doc.find_linked(TagToDoc).all(db).await.unwrap();
        for doc in select {
            println!("{:?}", doc);
        }
        // let tags = doc.find_related(DocAndTags).all(db).await.unwrap();
    }
}
