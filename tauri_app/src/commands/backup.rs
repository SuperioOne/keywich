use crate::{errors::AppErrors, result_log::ResultLog, AppDbState, DbNotifier, KeyState};
use keywich_lib::profile::backup::{
  file_backup::FileBackupReader, reader::BackupReader, BackupOptions, BackupTarget, RestoreOptions,
};
use serde::Serialize;
use std::{ops::Deref, path::Path};
use tauri::{AppHandle, State};

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
  is_valid: bool,
  path: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn backup_profile_db(
  state: State<'_, AppDbState>,
  key_state: State<'_, KeyState>,
  app: AppHandle,
  export_path: String,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;
  let local_data_dir = app
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)
    .log_err()?;

  let content_dir = Path::join(&local_data_dir, "contents");
  let password = match key_state.entry.get_password() {
    Ok(password) => password,
    Err(err @ keyring::Error::NoEntry) => {
      let _ = app.emit_unlock_required();
      return Err(err.into());
    }
    Err(err) => {
      return Err(err.into());
    }
  };

  if let Some(db) = read_lock.deref() {
    let options = BackupOptions {
      content_dir,
      target: BackupTarget::File(export_path.into()),
      sign_key: Box::from(password.as_bytes()),
    };

    db.backup(options).await.log_err()?;

    Ok(())
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn restore_profile_db(
  state: State<'_, AppDbState>,
  app: AppHandle,
  import_path: String,
) -> Result<(), AppErrors> {
  let read_lock = state.profile_db.read().await;
  let local_data_dir = app
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)
    .log_err()?;

  let content_dir = Path::join(&local_data_dir, "contents");

  if let Ok(false) = content_dir.try_exists() {
    std::fs::create_dir(&content_dir).map_err(|_| AppErrors::ContentPathFailed)?;
  }

  if let Some(db) = read_lock.deref() {
    let options = RestoreOptions {
      content_dir,
      target: BackupTarget::File(import_path.into()),
    };

    db.restore(options).await.log_err()?;

    Ok(())
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn verify_backup(
  app: AppHandle,
  key_state: State<'_, KeyState>,
  import_path: String,
) -> Result<VerifyResponse, AppErrors> {
  let password = match key_state.entry.get_password() {
    Ok(password) => password,
    Err(err @ keyring::Error::NoEntry) => {
      let _ = app.emit_unlock_required();
      return Err(err.into());
    }
    Err(err) => {
      return Err(err.into());
    }
  };

  let fd = std::fs::File::open(&import_path)
    .map_err(|err| AppErrors::BackupError(err.to_string()))
    .log_err()?;
  let mut reader = FileBackupReader::new(fd)?;
  let result = reader.verify_digest(password.as_bytes())?;

  Ok(VerifyResponse {
    path: import_path,
    is_valid: result,
  })
}
