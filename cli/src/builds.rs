use std::{fs::File, io::Write};

use anyhow::Result;

use api::web;

pub async fn save_build(path: String, data: web::ItemBuild) -> Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string(&data)?;
    f.write_all(&buf[..].as_bytes())?;
    Ok(())
}
