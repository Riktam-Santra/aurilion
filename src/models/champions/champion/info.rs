use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]

pub struct Info {
    pub attack: i16,
    pub defense: i16,
    pub magic: i16,
    pub difficulty: i16,
}
