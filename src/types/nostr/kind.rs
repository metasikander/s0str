use serde::Deserialize;
use serde::Serialize;

//-////////////////////////////////////////////////////////////////////////////
//  Kind
//-////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Kind {
    SetMetadata = 0,
    TextNote = 1,
    RecommendServer = 2,
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
