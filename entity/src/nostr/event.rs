use sea_orm::entity::prelude::*;
use crate::nostr::tag;

//-////////////////////////////////////////////////////////////////////////////
//  Event Model
//-////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "nostr_events")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: i16,
    pub content: String,
    pub sig: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "tag::Entity")]
    Tag,
}

impl Related<tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
