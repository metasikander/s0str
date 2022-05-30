use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    pub ids: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub kinds: Option<Vec<u64>>,
    pub e: Option<Vec<String>>,
    pub p: Option<Vec<String>>,
    pub since: Option<Vec<u64>>,
    pub until: Option<Vec<u64>>,
    pub limit: Option<u64>,
}
