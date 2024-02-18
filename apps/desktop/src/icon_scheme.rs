use crate::errors::AppErrors;
use const_format::concatcp;
use std::fs;
use std::fs::create_dir;
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
  let response = ResponseBuilder::new();
  let path = request.uri().strip_prefix(PREFIX).unwrap();
  let path = percent_encoding::percent_decode(path.as_bytes())
    .decode_utf8_lossy()
    .to_string();

  let local_data_dir = app
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::GenericError)?;

  let mut content_dir = Path::join(&local_data_dir, "contents");

  if !content_dir.exists() {
    create_dir(&content_dir).map_err(|_err| AppErrors::GenericError)?;
  }

  content_dir.push(&path);

  if content_dir.is_file() && content_dir.exists() {
    match fs::read(content_dir) {
      Ok(data) => response.mimetype("text/plain").status(200).body(data),
      Err(err) => response.status(400).body(err.to_string().into_bytes()),
    }
  } else {
    response.status(404).body(Vec::new())
  }
}
