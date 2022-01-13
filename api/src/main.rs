use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    label: String,
    value: String,
    isAram: Option<bool>,
    isUrf: Option<bool>,
}

#[tokio::main]
pub async fn main() {
  match fetch_source_list().await {
    Ok(list) => {
      println!("{:?}", list)
    }
    Err(e) => {
      println!("{:?}", e)
    }
  }
}

pub async fn fetch_source_list() -> Result<Vec<Source>, Box<dyn Error>> {
    let url = "https://cdn.jsdelivr.net/gh/champ-r/source-list/index.json11";
    match reqwest::get(url).await {
      Ok(resp) => {
        match resp.json::<Vec<Source>>().await {
          Ok(json) => {
            Ok(json)
          }
          Err(e) => {
            Result::Err(Box::new(e))
          }
        }
      }
      Err(e) => {
        Result::Err(Box::new(e))
      }
    }
}
