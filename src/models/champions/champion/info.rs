use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]

pub struct Info {
    pub attack: f32,
    pub defense: f32,
    pub magic: f32,
    pub difficulty: f32,
}
