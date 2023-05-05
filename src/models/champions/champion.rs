use serde::{Deserialize, Serialize};

use self::{image::Image, info::Info, stats::Stats};

pub mod image;
pub mod info;
pub mod stats;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Champion {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: Stats,
}
