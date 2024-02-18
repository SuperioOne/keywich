use crate::errors::AppErrors;
use image::imageops::FilterType;
use std::fs::create_dir;
use std::path::Path;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_config_path(handle: AppHandle) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::GenericError)?;

  let config_file = Path::join(&local_data_dir, "config.json");
  let path = config_file.to_str().ok_or(AppErrors::GenericError)?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_locale_path(handle: AppHandle, locale: String) -> Result<String, AppErrors> {
  let locale_data = handle
    .path_resolver()
    .resolve_resource(format!("locales/{}.json", locale))
    .ok_or(AppErrors::GenericError)?;

  let path = locale_data.to_str().ok_or(AppErrors::GenericError)?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upload_icon(handle: AppHandle, data: Vec<u8>) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::GenericError)?;

  let mut file_path = Path::join(&local_data_dir, "contents");

  if !file_path.exists() {
    create_dir(&file_path).map_err(|_err| AppErrors::GenericError)?;
  }

  let file_name = uuid::Uuid::now_v7().to_string();
  file_path.push(&file_name);

  let image_file = image::load_from_memory(&data).map_err(|err| AppErrors::GenericError)?;
  let resized = image_file.resize(128, 128, FilterType::Nearest);
  resized
    .save(&file_path)
    .map_err(|err| AppErrors::GenericError)?;

  Ok(file_name)
}
