use crate::errors::AppErrors;
use std::fs::create_dir;
use std::path::Path;
use tauri::AppHandle;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_guid() -> String {
  uuid::Uuid::now_v7().to_string()
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_content_path(handle: AppHandle, file_name: String) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::GenericError)?;

  let mut content_dir = Path::join(&local_data_dir, "contents");

  if !content_dir.exists() {
    create_dir(&content_dir).map_err(|_err| AppErrors::GenericError)?;
  }

  content_dir.push(&file_name);

  let path = content_dir.to_str().ok_or(AppErrors::GenericError)?;
  Ok(String::from(path))
}

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
