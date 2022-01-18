use anyhow::Result;

use api::web;
use futures::future::join_all;
use tokio::task::JoinHandle;

use crate::builds::save_build;

pub mod builds;

#[tokio::main]
pub async fn main() {
    println!("starting...");
    match apply_builds("op.gg".to_string()).await {
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
    let mut tasks: Vec<JoinHandle<Result<()>>> = vec![];

    for (champ_name, _champ_info) in champ_list.data.into_iter() {
        let source = source.clone();
        let npm_name = format!("@champ-r/{}", source);
        tasks.push(tokio::spawn(async move {
            let resp = web::fetch_champ_detail(
                npm_name,
                "latest".to_string(),
                champ_name.to_string(),
            )
            .await;

            let data = match resp {
                Ok(data) => match data {
                    Some(data) => data,
                    _ => vec![],
                },
                _ => vec![],
            };

            if data.len() == 0 {
                println!("not found: {}", champ_name);
            }

            for (idx, i) in data.iter().enumerate() {
                for (iidx, build) in i.item_builds.iter().enumerate() {
                    let p = format!("./.json/{}-{}-{}-{}.json", source, champ_name, idx, iidx);
                    match save_build(p, build).await {
                        Ok(_) => {
                            println!("saved: {}", champ_name);
                        },
                        Err(e) => (
                            println!("save err: {:?}", e)
                        ),
                    }
                }
            }

            Ok(())
        }));
    }

    join_all(tasks).await;
    Ok(())
}
