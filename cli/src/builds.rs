use std::{io::{Write, Error}, fs::File};

use api::web;

pub fn save_build(path: String, data: web::ItemBuild) -> Result<Error> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_string(&data)?;
    f.write_all(&buf[..])?
}
