use crate::errors::AppErrors;
use crate::AppRpcState;
use keywich_lib::profile::keys::{KeyData, KeyItem, SearchQuery};
use keywich_lib::profile::ProfileDB;
use tauri::{AppHandle, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_keys(state: State<'_, AppRpcState>) -> Result<Vec<KeyItem>, AppErrors> {
  let keys = state.profile_db.get_keys(false).await?;
  Ok(keys)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_pinned_keys(state: State<'_, AppRpcState>) -> Result<Vec<KeyItem>, AppErrors> {
  let keys = state.profile_db.get_keys(true).await?;
  Ok(keys)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_keys(
  state: State<'_, AppRpcState>,
  query: SearchQuery,
) -> Result<Vec<KeyItem>, AppErrors> {
  let keys = state.profile_db.search_keys(query).await?;
  Ok(keys)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_key(
  handle: AppHandle,
  state: State<'_, AppRpcState>,
  key_id: i64,
) -> Result<(), AppErrors> {
  delete_icon(&handle, &state.profile_db, key_id).await?;
  state.profile_db.delete_key(key_id).await?;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_key(state: State<'_, AppRpcState>, data: KeyData) -> Result<i64, AppErrors> {
  let result = state.profile_db.insert_key(data).await?;
  Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_key(
  handle: AppHandle,
  state: State<'_, AppRpcState>,
  key_id: i64,
  data: KeyData,
) -> Result<(), AppErrors> {
  delete_icon(&handle, &state.profile_db, key_id).await?;
  state.profile_db.update_key(key_id, data).await?;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn pin_key(state: State<'_, AppRpcState>, key_id: i64) -> Result<(), AppErrors> {
  state.profile_db.update_pin_status(key_id, true).await?;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn unpin_key(state: State<'_, AppRpcState>, key_id: i64) -> Result<(), AppErrors> {
  state.profile_db.update_pin_status(key_id, false).await?;
  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_key_by_id(
  state: State<'_, AppRpcState>,
  key_id: i64,
) -> Result<KeyItem, AppErrors> {
  if let Some(key) = state.profile_db.get_key_by_id(key_id).await? {
    Ok(key)
  } else {
    Err(AppErrors::GenericError)
  }
}

async fn delete_icon(
  handle: &AppHandle,
  profile_db: &ProfileDB,
  key_id: i64,
) -> Result<(), AppErrors> {
  if let Some(key) = profile_db.get_key_by_id(key_id).await? {
    if let Some(icon_id) = key.custom_icon {
      let local_data_dir = handle
        .path_resolver()
        .app_local_data_dir()
        .ok_or(AppErrors::GenericError)?;
      let mut dest_path = std::path::Path::join(&local_data_dir, "contents");

      if !dest_path.exists() {
        std::fs::create_dir(&dest_path).map_err(|_err| AppErrors::GenericError)?;
      }

      dest_path.push(icon_id);

      if dest_path.is_file() {
        let _result = std::fs::remove_file(dest_path);
      }
    }

    Ok(())
  } else {
    Err(AppErrors::GenericError)
  }
}
