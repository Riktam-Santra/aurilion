use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gold {
    pub base: f32,
    pub total: f32,
    pub sell: f32,
    pub purchasable: bool,
}
