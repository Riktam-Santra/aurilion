use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gold {
    pub base: i32,
    pub total: i32,
    pub sell: i32,
    pub purchasable: bool,
}
