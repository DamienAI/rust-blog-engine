use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub async fn asset_file(req: HttpRequest) -> Result<NamedFile> {
  let filename: String = req.match_info().query("filename").parse().unwrap();
  let path: PathBuf = format!("{}/src/articles/assets/{}", env!("CARGO_MANIFEST_DIR"), filename).parse().unwrap();
  Ok(NamedFile::open(path)?)
}
