use anyhow::Result;

use api::web;

use crate::builds::save_build;

pub mod builds;

#[tokio::main]
pub async fn main() {
    match apply_builds("@champ-r/op.gg".to_string()).await {
        Ok(_) => {
            println!("all set");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

pub async fn apply_builds(source: String) -> Result<()> {
    let v = web::fetch_lol_version_list().await?;
    let latest_version = v.first().unwrap();
    if latest_version.chars().count() == 0 {
        panic!("fetch lol version failed")
    }

    let champ_list = web::fetch_champ_list(latest_version.to_string()).await?;
    let mut not_found = vec![];

    for (champ_name, _champ_info) in champ_list.data.iter() {
        let data = web::fetch_champ_detail(
            source.to_string(),
            "latest".to_string(),
            champ_name.to_string(),
        )
        .await?;

        let data = match data {
            Some(d) => d,
            None => {
                not_found.push(champ_name.to_string());
                vec![]
            },
        };

        for (idx, i) in data.iter().enumerate() {
            for (iidx, build) in i.item_builds.iter().enumerate() {
                let p = format!("./.json/{}-{}-{}.json", champ_name, idx, iidx);
                save_build(p, build).await?;
            }
        }
    }

    println!("not found: {:?}", not_found);
    Ok(())
}
