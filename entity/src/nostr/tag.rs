use sea_orm::entity::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use crate::nostr::event;

//-////////////////////////////////////////////////////////////////////////////
//  Tag Model
//-////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "nostr_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub event_id: String,
    pub tag_type: TagType,
    pub hex: String,
    pub data: String,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TagType {
    #[sea_orm(string_value = "N")]
    #[serde(rename = "")]
    None,
    #[sea_orm(string_value = "e")]
    #[serde(rename = "e")]
    Event,
    #[sea_orm(string_value = "p")]
    #[serde(rename = "e")]
    Pubkey,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "event::Entity",
        from = "Column::EventId",
        to   = "event::Column::Id"
    )]
    Event,
}

impl Related<event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Event.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
