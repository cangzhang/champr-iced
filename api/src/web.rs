use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub label: String,
    pub value: String,
    pub is_aram: Option<bool>,
    pub is_urf: Option<bool>,
}

pub async fn fetch_source_list() -> Result<Vec<Source>, Box<dyn Error>> {
    let url = "https://cdn.jsdelivr.net/gh/champ-r/source-list/index.json";
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
    pub index: i64,
    pub id: String,
    pub version: String,
    pub official_version: String,
    pub timestamp: i64,
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
    pub associated_maps: Vec<i64>,
    pub associated_champions: Vec<i64>,
    pub blocks: Vec<Block>,
    pub map: String,
    pub mode: String,
    // pub preferred_item_slots: Vec<Value>,
    pub sortrank: i64,
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
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    pub alias: String,
    pub name: String,
    pub position: String,
    pub pick_count: i64,
    pub win_rate: String,
    pub primary_style_id: i64,
    pub sub_style_id: i64,
    pub selected_perk_ids: Vec<i64>,
    pub score: i64,
}

pub async fn fetch_champ_detail(
    source: String,
    version: String,
    champ_name: String,
) -> Result<Vec<ChampData>, Box<dyn Error>> {
    let url = format!(
        "https://cdn.jsdelivr.net/npm/{}@{}/{}.json",
        source, version, champ_name
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
    let url = format!("https://registry.npmmirror.com/{}/latest", source);
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<NpmInfo>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}

pub async fn fetch_version_list() -> Result<Vec<String>, Box<dyn Error>> {
    let url = "https://ddragon.leagueoflegends.com/api/versions.json";
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
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

pub async fn fetch_champ_list(version: String) -> Result<ChampListResp, Box<dyn Error>> {
    let url = format!(
        "http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json",
        version
    );
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<ChampListResp>().await {
            Ok(json) => Ok(json),
            Err(e) => Result::Err(Box::new(e)),
        },
        Err(e) => Result::Err(Box::new(e)),
    }
}
