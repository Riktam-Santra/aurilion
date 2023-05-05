pub mod basic;
pub mod group;
pub mod item_info;
pub mod tree;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use self::{basic::Basic, group::Group, item_info::ItemInfo, tree::Tree};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Items {
    #[serde(rename = "type")]
    pub data_type: String,
    pub version: String,
    pub basic: Basic,
    pub data: HashMap<String, ItemInfo>,
    pub groups: Vec<Group>,
    pub tree: Vec<Tree>,
}
