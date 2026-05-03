use std::fs;
use std::path::{Path, PathBuf};
use tauri::http::{Request, Response};
use tauri::{Manager as _, UriSchemeContext};

pub(crate) const ICON_PROTOCOL: &str = "kwicon";
pub(crate) const IMG_PROTOCOL: &str = "kwimg";

pub(crate) fn icon_protocol_handler<'a, R>(
  context: UriSchemeContext<'a, R>,
  request: Request<Vec<u8>>,
) -> Response<Vec<u8>>
where
  R: tauri::Runtime,
{
  let path = request.uri().path().trim_start_matches("/");
  let path = percent_encoding::percent_decode(path.as_bytes())
    .decode_utf8_lossy()
    .to_string();
  let response = Response::builder();

  let local_data_dir = match context.app_handle().path().app_local_data_dir() {
    Ok(v) => v,
    Err(err) => {
      return response
        .status(400)
        .body(err.to_string().into_bytes())
        .expect("");
    }
  };

  let mut icon_path = Path::join(&local_data_dir, "contents");
  icon_path.push(&path);

  if icon_path.is_file() {
    match fs::read(icon_path) {
      Ok(data) => response.status(200).body(data).expect(""),
      Err(err) => response
        .status(400)
        .body(err.to_string().into_bytes())
        .expect(""),
    }
  } else {
    response.status(404).body(Vec::new()).expect("")
  }
}

pub(crate) fn img_protocol_handler<'a, R>(
  _app: UriSchemeContext<'a, R>,
  request: Request<Vec<u8>>,
) -> Response<Vec<u8>>
where
  R: tauri::Runtime,
{
  let path = request.uri().path();
  let path = percent_encoding::percent_decode(path.as_bytes())
    .decode_utf8_lossy()
    .to_string();
  let path = PathBuf::from(path);
  let response = Response::builder();

  if path.is_file() {
    match fs::read(path) {
      Ok(data) => response.status(200).body(data).expect(""),
      Err(err) => response
        .status(400)
        .body(err.to_string().into_bytes())
        .expect(""),
    }
  } else {
    response.status(404).body(Vec::new()).expect("")
  }
}
