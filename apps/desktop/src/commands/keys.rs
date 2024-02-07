use crate::errors::AppErrors;
use crate::AppRpcState;
use keywich_lib::profile::keys::{KeyData, KeyItem, SearchQuery};
use tauri::State;

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
pub async fn delete_key(state: State<'_, AppRpcState>, key_id: i64) -> Result<(), AppErrors> {
  if state.profile_db.delete_key(key_id).await? {
    Ok(())
  } else {
    Err(AppErrors::GenericError)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_key(state: State<'_, AppRpcState>, data: KeyData) -> Result<i64, AppErrors> {
  let result = state.profile_db.insert_key(data).await?;
  Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_key(
  state: State<'_, AppRpcState>,
  key_id: i64,
  data: KeyData,
) -> Result<(), AppErrors> {
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
