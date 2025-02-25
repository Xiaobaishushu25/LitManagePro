use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, DeriveEntityModel, Serialize,Deserialize)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    pub index: i32,// 索引，排序用的
    pub group_id: i32,
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub value: String,
    pub bg_color: String,
    pub text_color: String,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::tag_group::Entity", from = "Column::GroupId", to = "super::tag_group::Column::Id")]
    TagGroup,
}

impl Related<super::tag_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
