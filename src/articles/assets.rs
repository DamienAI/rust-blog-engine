use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

/// Controller for the assets endpoint.
///
/// Will load the file referenced in the request and return its content.
/// 
/// # Responses
/// 
/// - 200 Default successful response with file content as binary blob.
/// 
/// - 404 The asset does not exist
/// 
/// # Panics
/// 
/// Panics will return a 404.
///
pub async fn asset_file(req: HttpRequest) -> Result<NamedFile> {
  let filename: String = req.match_info().query("filename").parse().unwrap();
  let path: PathBuf = format!("{}/src/articles/assets/{}", env!("CARGO_MANIFEST_DIR"), filename).parse().unwrap();
  Ok(NamedFile::open(path)?)
}
