use sea_orm::ActiveModelTrait;
use sea_orm::DbConn;
use sea_orm::DbErr;
use sea_orm::entity::Set;
use sea_orm::TransactionTrait;
use serde::Deserialize;
use serde::Serialize;
use crate::types::nostr::tag::Tag;
use entity::nostr::event;
use entity::nostr::tag;

//-////////////////////////////////////////////////////////////////////////////
//  Event
//-////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: i16,
    pub tags: Vec<Tag>,
    pub content: String,
    pub sig: String,
}

impl Event {

    // --- Database ---------------------------------------

    pub async fn insert(self, db: &DbConn) {
        // convert
        let id = self.id.clone();
        let (event, tags) = self.to_model();
        let tags: Vec<tag::ActiveModel> = tags.into_iter()
            .map(|tag| tag.to_model(id.clone()))
            .collect();
        // insert
        db.transaction::<_, (), DbErr>(|txn| {
            // rust
            Box::pin(async move {
                event.insert(txn).await?;
                for tag in tags {
                    tag.insert(txn).await?;
                }
                Ok(())
            })
        }).await.unwrap();
    }

    pub fn to_model(self) -> (event::ActiveModel, Vec<Tag>) {
        (
            event::ActiveModel{
                id: Set(self.id),
                pubkey: Set(self.pubkey),
                created_at: Set(self.created_at),
                kind: Set(self.kind),
                content: Set(self.content),
                sig: Set(self.sig),
            },
            self.tags,
        )
    }

    pub fn from_model(model: event::Model, tags: Vec<Tag>) -> Event {
        Event {
            id: model.id,
            pubkey: model.pubkey,
            created_at: model.created_at,
            kind: model.kind,
            tags,
            content: model.content,
            sig: model.sig,
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
