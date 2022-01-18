use anyhow::Result;

use api::web;
use futures::StreamExt;

use crate::builds::save_build;

pub mod builds;

#[tokio::main]
pub async fn main() {
    let sources = vec!["op.gg".to_string(), "op.gg-aram".to_string()];
    let folder = "./.json".to_string();

    println!("starting...");
    match apply_builds(sources, folder).await {
        Ok(_) => {
            println!("all set");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

pub async fn apply_builds(sources: Vec<String>, path: String) -> Result<()> {
    let v = web::fetch_lol_version_list().await?;
    let latest_version = v.first().unwrap();
    if latest_version.chars().count() == 0 {
        panic!("fetch lol version failed")
    }

    let champ_list = web::fetch_champ_list(latest_version.to_string()).await?;
    let mut tasks = vec![];

    for (champ_name, _champ_info) in champ_list.data.into_iter() {
        for source in sources.iter() {
            let source = source.clone();
            let champ_name = champ_name.clone();
            let npm_name = format!("@champ-r/{}", source);
            let path = path.clone();

            tasks.push(async move {
                let resp =
                    web::fetch_champ_detail(npm_name, "latest".to_string(), champ_name.to_string())
                        .await;

                let data = match resp {
                    Ok(data) => match data {
                        Some(data) => data,
                        _ => vec![],
                    },
                    _ => vec![],
                };

                if data.len() == 0 {
                    println!("failed: {} {}", source, champ_name);
                }

                for (idx, i) in data.iter().enumerate() {
                    for (iidx, build) in i.item_builds.iter().enumerate() {
                        let p = format!(
                            "{path}/{source}-{champ_name}-{idx}-{iidx}.json",
                            path = path,
                            source = source,
                            champ_name = champ_name,
                            idx = idx,
                            iidx = iidx
                        );
                        match save_build(p, build).await {
                            Ok(_) => {
                                println!("saved: {} {}", source, champ_name);
                            }
                            Err(e) => {
                                println!("save err: {:?}", e);
                            }
                        }
                    }
                }
            });
        }
    }

    futures::stream::iter(tasks)
        .buffer_unordered(10)
        .collect::<Vec<()>>()
        .await;
    Ok(())
}
