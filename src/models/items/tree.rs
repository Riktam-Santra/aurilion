use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tree {
    pub header: String,
    pub tags: Vec<String>,
}
