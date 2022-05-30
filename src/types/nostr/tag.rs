use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Tag {
    #[serde(rename = "e")]
    Event,
    #[serde(rename = "p")]
    Pubkey,
}
