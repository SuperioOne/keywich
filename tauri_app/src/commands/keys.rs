use crate::errors::AppErrors;
use crate::{AppDbState, DbNotifier};
use keywich_lib::profile::keys::{KeyData, KeyItem, SearchQuery};
use std::ops::Deref;
use tauri::{AppHandle, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_keys(
  state: State<'_, AppDbState>,
  app: AppHandle,
) -> Result<Vec<KeyItem>, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    let keys = profile_db.get_keys(false).await?;
    Ok(keys)
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_pinned_keys(
  state: State<'_, AppDbState>,
  app: AppHandle,
) -> Result<Vec<KeyItem>, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    let keys = profile_db.get_keys(true).await?;
    Ok(keys)
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_keys(
  state: State<'_, AppDbState>,
  app: AppHandle,
  query: SearchQuery,
) -> Result<Vec<KeyItem>, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    let keys = profile_db.search_keys(query).await?;
    Ok(keys)
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_key(
  state: State<'_, AppDbState>,
  app: AppHandle,
  key_id: i64,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    if let Some(key_data) = profile_db.get_key_by_id(key_id).await? {
      if let Some(icon_name) = key_data.custom_icon {
        delete_icon(&app, &icon_name)?;
      }
      profile_db.delete_key(key_id).await?;
      Ok(())
    } else {
      Err(AppErrors::KeyNotFound)
    }
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_key(
  state: State<'_, AppDbState>,
  app: AppHandle,
  data: KeyData,
) -> Result<i64, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    let result = profile_db.insert_key(data).await?;
    Ok(result)
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_key(
  app: AppHandle,
  state: State<'_, AppDbState>,
  key_id: i64,
  data: KeyData,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    if let Some(key_data) = profile_db.get_key_by_id(key_id).await? {
      match &key_data.custom_icon {
        ic @ Some(icon_name) if ic.ne(&data.custom_icon) => {
          delete_icon(&app, &icon_name)?;
        }
        _ => {}
      }
      profile_db.update_key(key_id, data).await?;
      Ok(())
    } else {
      Err(AppErrors::KeyNotFound)
    }
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn pin_key(
  state: State<'_, AppDbState>,
  app: AppHandle,
  key_id: i64,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    profile_db.update_pin_status(key_id, true).await?;
    Ok(())
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn unpin_key(
  state: State<'_, AppDbState>,
  app: AppHandle,
  key_id: i64,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    profile_db.update_pin_status(key_id, false).await?;
    Ok(())
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_key_by_id(
  state: State<'_, AppDbState>,
  app: AppHandle,
  key_id: i64,
) -> Result<KeyItem, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    if let Some(key) = profile_db.get_key_by_id(key_id).await? {
      Ok(key)
    } else {
      Err(AppErrors::KeyNotFound)
    }
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

fn delete_icon(handle: &AppHandle, icon_name: &str) -> Result<(), AppErrors> {
  let mut dest_path = handle
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)?;

  dest_path.push("contents");
  dest_path.push(icon_name);

  if dest_path.is_file() {
    let _result = std::fs::remove_file(dest_path);
  }

  Ok(())
}
