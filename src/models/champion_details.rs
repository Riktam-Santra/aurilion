use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::champions::champion::{image::Image, info::Info, stats::Stats};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChampionDetails {
    #[serde(rename = "type")]
    pub data_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampionData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChampionData {
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub image: Image,
    pub skins: Vec<Skin>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub tags: Vec<String>,
    pub partype: String,
    pub info: Info,
    pub stats: Stats,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Skin {
    pub id: String,
    pub num: f32,
    pub name: String,
    pub chromas: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: LevelTip,
    pub maxrank: f32,
    pub cooldown: Vec<f32>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<f32>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub datavalues: Value,
    pub effect: Vec<Option<Vec<f32>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Value,
    #[serde(rename = "costType")]
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<f32>,
    #[serde(rename = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LevelTip {
    pub label: Vec<String>,
    pub effect: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Passive {
    pub name: String,
    pub description: String,
    pub image: Image,
}
