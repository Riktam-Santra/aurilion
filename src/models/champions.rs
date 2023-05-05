use std::collections::HashMap;

use self::champion::Champion;
use serde::{Deserialize, Serialize};

pub mod champion;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Champions {
    #[serde(rename = "type")]
    pub data_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
}
