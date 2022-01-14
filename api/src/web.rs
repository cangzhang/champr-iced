use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub const CDN_JSDELIVR: &str = "https://cdn.jsdelivr.net";
pub const NPM_MIRROR: &str = "https://registry.npmmirror.com";
pub const CDN_DDRAGON: &str = "https://ddragon.leagueoflegends.com";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub label: String,
    pub value: String,
    pub is_aram: Option<bool>,
    pub is_urf: Option<bool>,
}

pub async fn fetch_source_list() -> Result<Vec<Source>, Box<dyn Error>> {
    let url = format!(
        "{cdn}/gh/champ-r/source-list/index.json",
        cdn = CDN_JSDELIVR
    );
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<Vec<Source>>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampData {
    pub index: u32,
    pub id: String,
    pub version: String,
    pub official_version: String,
    pub timestamp: u64,
    pub alias: String,
    pub name: String,
    pub position: String,
    pub skills: Vec<String>,
    pub spells: Vec<String>,
    pub item_builds: Vec<ItemBuild>,
    pub runes: Vec<Rune>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemBuild {
    pub title: String,
    pub associated_maps: Vec<u32>,
    pub associated_champions: Vec<u32>,
    pub blocks: Vec<Block>,
    pub map: String,
    pub mode: String,
    // pub preferred_item_slots: Vec<Value>,
    pub sortrank: u32,
    pub started_from: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "type")]
    pub type_field: String,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub count: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub alias: String,
    pub name: String,
    pub position: String,
    pub pick_count: u64,
    pub win_rate: String,
    pub primary_style_id: u32,
    pub sub_style_id: u32,
    pub selected_perk_ids: Vec<u32>,
    pub score: f64,
}

pub async fn fetch_champ_detail(
    source: String,
    version: String,
    champ_name: String,
) -> Result<Vec<ChampData>, Box<dyn Error>> {
    let url = format!(
        "{cdn}/{source}@{version}/{champ_name}.json",
        cdn = CDN_JSDELIVR,
        source = &source,
        version = &version,
        champ_name = &champ_name
    );
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<Vec<ChampData>>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpmInfo {
    pub name: String,
    pub version: String,
    pub source_version: String,
    pub description: String,
    pub main: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistTags {
    pub latest: String,
}

pub async fn fetch_npm_info(source: String) -> Result<NpmInfo, Box<dyn Error>> {
    let url = format!("{cdn}/{source}/latest", cdn = NPM_MIRROR, source = &source);
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<NpmInfo>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}

pub async fn fetch_version_list() -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{cdn}/api/versions.json", cdn = CDN_DDRAGON);
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<Vec<String>>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampListResp {
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, ChampInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampInfo {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    // pub blurb: String,
    // pub info: Info,
    pub image: Image,
    pub tags: Vec<String>,
    // pub partype: String,
    // pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub async fn fetch_champ_list(version: String) -> Result<ChampListResp, Box<dyn Error>> {
    let url = format!(
        "{cdn}/cdn/{version}/data/en_US/champion.json",
        cdn = CDN_DDRAGON,
        version = &version
    );
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<ChampListResp>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}
