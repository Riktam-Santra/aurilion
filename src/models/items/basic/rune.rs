use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rune {
    pub isrune: bool,
    pub tier: i32,
    #[serde(rename = "type")]
    pub rune_type: String,
}
