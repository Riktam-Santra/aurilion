use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::champions::champion::image::Image;

use super::basic::gold::Gold;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemInfo {
    pub name: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    // pub into: Vec<String>,
    pub image: Image,
    pub gold: Gold,
    pub tags: Vec<String>,
    pub maps: HashMap<String, bool>,
    pub stats: Value,
}
