use crate::errors::AppErrors;
use crate::AppDbState;
use image::imageops::FilterType;
use image::ImageFormat;
use std::collections::HashSet;
use tauri::{AppHandle, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_config_path(handle: AppHandle) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)?;

  let config_file = std::path::Path::join(&local_data_dir, "config.json");
  let path = config_file.to_str().ok_or(AppErrors::ConfigPathFailed)?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_locale_path(handle: AppHandle, locale: String) -> Result<String, AppErrors> {
  let locale_path = handle
    .path_resolver()
    .resolve_resource(format!("locales/{}.json", locale))
    .ok_or(AppErrors::LocalDataDirNotFound)?;

  let path = locale_path.to_str().ok_or(AppErrors::LocalePathFailed)?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn process_icon(handle: AppHandle, file_path: String) -> Result<String, AppErrors> {
  let src_path = std::path::Path::new(&file_path);

  if src_path.is_file() {
    let src_file =
      std::fs::read(src_path).map_err(|err| AppErrors::IconReadFailed(err.to_string()))?;

    let image_file = image::load_from_memory(&src_file)
      .map_err(|err| AppErrors::IconReadFailed(err.to_string()))?;

    let file_name = uuid::Uuid::now_v7().to_string();
    let local_data_dir = handle
      .path_resolver()
      .app_local_data_dir()
      .ok_or(AppErrors::LocalDataDirNotFound)?;
    let mut dest_path = std::path::Path::join(&local_data_dir, "contents");

    if !dest_path.exists() {
      std::fs::create_dir(&dest_path).map_err(|_err| AppErrors::ContentPathFailed)?;
    }

    dest_path.push(&file_name);

    let resized = image_file.resize(128, 128, FilterType::Triangle);
    resized
      .save_with_format(&dest_path, ImageFormat::Png)
      .map_err(|err| AppErrors::IconResizeFailed(err.to_string()))?;

    Ok(file_name)
  } else {
    Err(AppErrors::IconReadFailed(format!(
      "{} is not a file",
      file_path
    )))
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn alloc_temp_path(handle: AppHandle) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)?;
  let mut temp_path = std::path::Path::join(&local_data_dir, "temp");

  if !temp_path.exists() {
    std::fs::create_dir(&temp_path).map_err(|_err| AppErrors::TempFolderFailed)?;
  }

  let file_name = uuid::Uuid::now_v7().to_string();
  temp_path.push(&file_name);
  let path = temp_path.to_str().ok_or(AppErrors::TempFolderFailed)?;

  Ok(String::from(path))
}
