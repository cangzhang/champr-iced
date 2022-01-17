use std::{fs::{File, self}, io::Write, path::Path};

use anyhow::Result;

use api::web;

pub async fn save_build(path: String, data: &web::ItemBuild) -> Result<()> {
    let path = Path::new(&path);
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    
    let mut f = File::create(&path)?;
    let buf = serde_json::to_string(&data)?;
    f.write_all(&buf[..].as_bytes())?;
    Ok(())
}
