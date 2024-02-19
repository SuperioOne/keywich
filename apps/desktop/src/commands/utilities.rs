use crate::errors::AppErrors;
use crate::AppRpcState;
use image::imageops::FilterType;
use image::ImageFormat;
use std::collections::HashSet;
use tauri::{AppHandle, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_config_path(handle: AppHandle) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::GenericError)?;

  let config_file = std::path::Path::join(&local_data_dir, "config.json");
  let path = config_file.to_str().ok_or(AppErrors::GenericError)?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_locale_path(handle: AppHandle, locale: String) -> Result<String, AppErrors> {
  let locale_path = handle
    .path_resolver()
    .resolve_resource(format!("locales/{}.json", locale))
    .ok_or(AppErrors::GenericError)?;

  let path = locale_path.to_str().ok_or(AppErrors::GenericError)?;

  Ok(String::from(path))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn process_icon(handle: AppHandle, file_path: String) -> Result<String, AppErrors> {
  println!("{}", &file_path);
  let src_path = std::path::Path::new(&file_path);

  if src_path.is_file() {
    let src_file = std::fs::read(src_path).map_err(|err| AppErrors::GenericError)?;
    let image_file = image::load_from_memory(&src_file).map_err(|err| {
      println!("{:?}", err);
      AppErrors::GenericError
    })?;

    let file_name = uuid::Uuid::now_v7().to_string();
    let local_data_dir = handle
      .path_resolver()
      .app_local_data_dir()
      .ok_or(AppErrors::GenericError)?;
    let mut dest_path = std::path::Path::join(&local_data_dir, "contents");

    if !dest_path.exists() {
      std::fs::create_dir(&dest_path).map_err(|_err| AppErrors::GenericError)?;
    }

    dest_path.push(&file_name);

    let resized = image_file.resize(128, 128, FilterType::Triangle);
    println!("Image resized");
    resized
      .save_with_format(&dest_path, ImageFormat::Png)
      .map_err(|err| {
        println!("{:?}", err);
        AppErrors::GenericError
      })?;

    Ok(file_name)
  } else {
    Err(AppErrors::GenericError)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn alloc_temp_path(handle: AppHandle) -> Result<String, AppErrors> {
  let local_data_dir = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::GenericError)?;
  let mut temp_path = std::path::Path::join(&local_data_dir, "temp");

  if !temp_path.exists() {
    std::fs::create_dir(&temp_path).map_err(|_err| AppErrors::GenericError)?;
  }

  let file_name = uuid::Uuid::now_v7().to_string();
  temp_path.push(&file_name);
  let path = temp_path.to_str().ok_or(AppErrors::GenericError)?;

  Ok(String::from(path))
}
