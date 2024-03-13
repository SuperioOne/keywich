use crate::errors::AppErrors;
use crate::{AppDbState, DbNotifier};
use keywich_lib::profile::charsets::CharsetItem;
use std::ops::Deref;
use tauri::{AppHandle, State};
use crate::result_log::ResultLog;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_charsets(
  state: State<'_, AppDbState>,
  app: AppHandle,
) -> Result<Vec<CharsetItem>, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    let result = profile_db.get_charsets().await.log_err()?;
    Ok(result)
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_charset(
  state: State<'_, AppDbState>,
  app: AppHandle,
  charset: CharsetItem,
) -> Result<String, AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    let result = profile_db.insert_charset(charset).await.log_err()?;
    Ok(result)
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_charset(
  state: State<'_, AppDbState>,
  app: AppHandle,
  name: String,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    profile_db.delete_charset(&name).await.log_err()?;
    Ok(())
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}
