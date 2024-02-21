use crate::errors::AppErrors;
use const_format::concatcp;
use std::fs;
use std::path::Path;
use tauri::http::{Request, Response, ResponseBuilder};
use tauri::AppHandle;

pub(crate) const ICON_PROTOCOL: &'static str = "kwicon";

const PREFIX: &'static str = concatcp!("", ICON_PROTOCOL, "://localhost/");

pub(crate) fn icon_protocol_handler<R>(
  app: &AppHandle<R>,
  request: &Request,
) -> Result<Response, Box<dyn std::error::Error>>
where
  R: tauri::Runtime,
{
  let path = request.uri().strip_prefix(PREFIX).unwrap();
  let path = percent_encoding::percent_decode(path.as_bytes())
    .decode_utf8_lossy()
    .to_string();

  let local_data_dir = app
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)?;

  let mut icon_path = Path::join(&local_data_dir, "contents");
  icon_path.push(&path);

  let response = ResponseBuilder::new();
  if icon_path.is_file() && icon_path.exists() {
    match fs::read(icon_path) {
      Ok(data) => response.status(200).body(data),
      Err(err) => response.status(400).body(err.to_string().into_bytes()),
    }
  } else {
    response.status(404).body(Vec::new())
  }
}
