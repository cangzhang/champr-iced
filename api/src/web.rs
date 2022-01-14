use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    label: String,
    value: String,
    isAram: Option<bool>,
    isUrf: Option<bool>,
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
    // pub author: Author,
    // pub license: String,
    // pub git_head: String,
    // #[serde(rename = "_id")]
    // pub id: String,
    // #[serde(rename = "_nodeVersion")]
    // pub node_version: String,
    // #[serde(rename = "_npmVersion")]
    // pub npm_version: String,
    // pub dist: Dist,
    // #[serde(rename = "_npmUser")]
    // pub npm_user: NpmUser,
    // pub directories: Directories,
    // pub maintainers: Vec<Maintainer>,
    // #[serde(rename = "_npmOperationalInternal")]
    // pub npm_operational_internal: NpmOperationalInternal,
    // #[serde(rename = "_hasShrinkwrap")]
    // pub has_shrinkwrap: bool,
    // #[serde(rename = "_cnpmcore_publish_time")]
    // pub cnpmcore_publish_time: String,
    // #[serde(rename = "publish_time")]
    // pub publish_time: i64,
    // #[serde(rename = "_cnpm_publish_time")]
    // pub cnpm_publish_time: i64,
    // pub readme: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
    pub file_count: i64,
    pub unpacked_size: i64,
    #[serde(rename = "npm-signature")]
    pub npm_signature: String,
    pub size: i64,
    pub noattachment: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpmUser {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directories {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpmOperationalInternal {
    pub host: String,
    pub tmp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistTags {
    pub latest: String,
}

pub async fn fetch_npm_info(source: String) -> Result<NpmInfo, Box<dyn Error>> {
    let url = format!("https://registry.npmmirror.com/{}/latest", source);
    match reqwest::get(url).await {
      Ok(resp) => {
        match resp.json::<NpmInfo>().await {
          Ok(json) => Ok(json),
          Err(e) => Result::Err(Box::new(e))
        }
      }
      Err(e) => Result::Err(Box::new(e))
    }
}
