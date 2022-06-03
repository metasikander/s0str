use entity::nostr::tag;
use entity::nostr::tag::TagType;
use sea_orm::entity::NotSet;
use sea_orm::entity::Set;
use serde_tuple::Deserialize_tuple;
use serde_tuple::Serialize_tuple;

//-////////////////////////////////////////////////////////////////////////////
//  Tag
//-////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Clone, Debug, Deserialize_tuple, Serialize_tuple)]
pub struct Tag {
    pub tag_type: TagType,
    pub hex: String,
    pub data: String,
}

impl Tag {
    pub fn to_model(self, event_id: String) -> tag::ActiveModel {
        tag::ActiveModel {
            event_id: Set(event_id),
            tag_type: Set(self.tag_type),
            hex: Set(self.hex),
            data: Set(self.data),
            id: NotSet,
        }
    }

    pub fn from_model(model: tag::Model) -> Tag {
        Tag {
            tag_type: model.tag_type,
            hex: model.hex,
            data: model.data,
        }
    }
}

//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
