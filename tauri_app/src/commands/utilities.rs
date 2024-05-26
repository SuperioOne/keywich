use crate::errors::AppErrors;
use crate::result_log::ResultLog;
use crate::LogLevel;
use image::imageops::FilterType;
use image::ImageFormat;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, State};

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
  pub color_theme: Option<String>,
  pub locale: Option<String>,
  pub is_light_theme: Option<bool>,
}

#[derive(Serialize)]
pub struct AppConfig {
  pub configs: Option<ConfigFile>,
  pub is_db_created: bool,
  pub locale_keys: Option<HashMap<String, String>>,
  pub available_locales: Vec<String>,
  pub log_level: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_config_path(handle: AppHandle) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)?;

  let config_file = Path::join(&local_data_dir, "config.json");
  let path = config_file
    .to_str()
    .ok_or(AppErrors::ConfigPathFailed)
    .log_err()?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_locale_path(handle: AppHandle, locale: String) -> Result<String, AppErrors> {
  let locale_path = handle
    .path_resolver()
    .resolve_resource(format!("locales/{}.json", locale))
    .ok_or(AppErrors::LocalDataDirNotFound)
    .log_err()?;

  let path = locale_path
    .to_str()
    .ok_or(AppErrors::LocalePathFailed)
    .log_err()?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn process_icon(handle: AppHandle, file_path: String) -> Result<String, AppErrors> {
  let src_path = Path::new(&file_path);

  if src_path.is_file() {
    let src_file = fs::read(src_path)
      .map_err(|err| AppErrors::IconReadFailed(err.to_string()))
      .log_err()?;

    let image_file = image::load_from_memory(&src_file)
      .map_err(|err| AppErrors::IconReadFailed(err.to_string()))
      .log_err()?;

    let file_name = uuid::Uuid::now_v7().to_string();
    let local_data_dir = handle
      .path_resolver()
      .app_local_data_dir()
      .ok_or(AppErrors::LocalDataDirNotFound)
      .log_err()?;

    let mut dest_path = Path::join(&local_data_dir, "contents");

    if let Ok(false) = dest_path.try_exists() {
      fs::create_dir(&dest_path)
        .map_err(|_err| AppErrors::ContentPathFailed)
        .log_err()?;
    };

    dest_path.push(&file_name);

    let resized = image_file.resize(128, 128, FilterType::Triangle);
    resized
      .save_with_format(&dest_path, ImageFormat::Png)
      .map_err(|err| AppErrors::IconResizeFailed(err.to_string()))
      .log_err()?;

    Ok(file_name)
  } else {
    Err(AppErrors::IconReadFailed(format!(
      "{} is not a file",
      file_path
    )))
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn load_configs(
  app: AppHandle,
  log_level: State<'_, LogLevel>,
) -> Result<AppConfig, AppErrors> {
  let local_data_dir = app
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)
    .log_err()?;

  let db_path = Path::join(&local_data_dir, crate::commands::login::APP_DB_NAME).metadata();
  let config_file = Path::join(&local_data_dir, "config.json");
  let mut app_details = AppConfig {
    is_db_created: db_path.is_ok_and(|metadata| metadata.is_file() && metadata.len() > 0),
    configs: None,
    locale_keys: None,
    // TODO: Convert this to proc macro and generate from locales folder automatically when there are more than 5 locale. For now it's unnecessary.
    available_locales: vec![String::from("en"), String::from("tr")],
    log_level: log_level.level.to_string(),
  };

  match read_json_file::<ConfigFile, _>(config_file) {
    Ok(config) => {
      app_details.configs = Some(config);
    }
    Err(_) => {
      error!("Unable to read config.");
    }
  }

  if let Some(cfg) = &app_details.configs {
    if let Some(locale) = &cfg.locale {
      if let Some(locale_path) = app
        .path_resolver()
        .resolve_resource(format!("locales/{}.json", locale))
      {
        match read_json_file::<HashMap<String, String>, _>(locale_path) {
          Ok(locale_map) => {
            app_details.locale_keys = Some(locale_map);
          }
          Err(_) => {
            error!("Unable to read locales.");
          }
        }
      }
    }
  }

  Ok(app_details)
}

enum FileErrors {
  UnableToOpenFile(String),
  DeserializerError(String),
}

fn read_json_file<T, P>(path: P) -> Result<T, FileErrors>
where
  P: AsRef<Path>,
  T: serde::de::DeserializeOwned,
{
  let file = fs::File::open(path).map_err(|e| FileErrors::UnableToOpenFile(e.to_string()))?;
  let parsed = serde_json::from_reader::<_, T>(&file)
    .map_err(|e| FileErrors::DeserializerError(e.to_string()))?;

  Ok(parsed)
}
