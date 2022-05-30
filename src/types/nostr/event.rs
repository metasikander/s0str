use serde::Deserialize;
use serde::Serialize;
use crate::types::nostr::tag::Tag;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u8,
    pub tags: Vec<(Option<Tag>, String, String)>,
    pub content: String,
    pub sig: String,
}
