use std::{
    fs::{self, File},
    io::Write,
    path::Path, sync::mpsc,
};

use anyhow::Result;
use futures::StreamExt;

use crate::web;

pub async fn save_build(path: String, data: &web::ItemBuild) -> Result<()> {
    let path = Path::new(&path);
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();

    let mut f = File::create(&path)?;
    let buf = serde_json::to_string(&data)?;
    f.write_all(&buf[..].as_bytes())?;
    Ok(())
}

pub async fn apply_builds(
    sources: Vec<String>,
    path: String,
    keep_old: bool,
) -> Result<Vec<(bool, String, String)>> {
    let path_exists = Path::new(&path).exists();
    if path_exists && !keep_old {
        fs::remove_dir_all(path.clone())?;
        println!("emptied old dir: {}", path);
    }

    let v = web::fetch_lol_version_list().await?;
    let latest_version = v.first().unwrap();
    if latest_version.chars().count() == 0 {
        panic!("fetch lol version failed")
    }

    let champ_list = web::fetch_champ_list(latest_version.to_string()).await?;
    let mut tasks = vec![];

    let (tx, rx) = mpsc::channel();

    for (champ_name, _champ_info) in champ_list.data.into_iter() {
        for source in sources.iter() {
            let source = source.clone();
            let champ_name = champ_name.clone();
            let npm_name = format!("@champ-r/{}", source);
            let path = path.clone();

            let tx = tx.clone();

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
                    tx.send((false, source.clone(), champ_name.clone()))
                        .unwrap();
                    println!("failed: {} {}", source, champ_name);
                }

                for (idx, i) in data.iter().enumerate() {
                    for (iidx, build) in i.item_builds.iter().enumerate() {
                        let p = format!(
                            "{path}/{champ_name}/{source}-{champ_name}-{idx}-{iidx}.json",
                            path = path,
                            source = source,
                            champ_name = champ_name,
                            idx = idx,
                            iidx = iidx
                        );
                        match save_build(p, build).await {
                            Ok(_) => {
                                // println!("finished: [{}] {}", source, champ_name);
                                tx.send((true, source.clone(), champ_name.clone())).unwrap();
                            }
                            Err(e) => {
                                println!("save err: {:?}", e);
                                tx.send((false, source.clone(), champ_name.clone()))
                                    .unwrap();
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

    drop(tx);

    let mut results: Vec<(bool, String, String)> = vec![];
    for r in rx {
        if r.0 == false {
            println!("{:?}", r);
        }
        results.push(r);
    }
    println!("all {}", results.len());

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn save_build() {
        let sources = vec![
            "op.gg-aram".to_string(),
            "op.gg".to_string(),
            "lolalytics".to_string(),
            "lolalytics-aram".to_string(),
        ];
        let folder = "../.json".to_string();
        let keep_old = false;

        println!(
            "start: save builds to local, sources: {:?}, keep old items: {}",
            sources, keep_old
        );

        match apply_builds(sources, folder, keep_old).await {
            Ok(_) => {
                println!("all set");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
