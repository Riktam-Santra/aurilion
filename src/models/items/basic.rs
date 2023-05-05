pub mod gold;
pub mod rune;
pub mod stats;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::{gold::Gold, rune::Rune, stats::Stats};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basic {
    pub name: String,
    pub rune: Rune,
    pub gold: Gold,
    pub group: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    pub consumed: bool,
    pub stacks: i32,
    pub depth: i32,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: bool,
    pub from: Value,
    pub into: Value,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: i32,
    #[serde(rename = "inStore")]
    pub in_store: bool,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: bool,
    #[serde(rename = "requiredChampion")]
    pub required_champion: String,
    #[serde(rename = "requiredAlly")]
    pub required_ally: String,
    pub stats: Stats,
    pub tags: Vec<String>,
    pub maps: HashMap<String, bool>,
}
