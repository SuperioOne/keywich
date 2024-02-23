use crate::errors::AppErrors;
use crate::{AppDbState, DbNotifier, KeyState};
use keywich_lib::profile::{ProfileDB, ProfileDBSqliteOptions, SqlitePassphrase};
use keywich_lib::scrypt::{scrypt, Params};
use std::path::Path;
use tauri::{AppHandle, State};

#[tauri::command(rename_all = "snake_case")]
pub async fn unlock_db(
  state: State<'_, AppDbState>,
  key_state: State<'_, KeyState>,
  app: AppHandle,
  master_pass: String,
) -> Result<(), AppErrors> {
  key_state.entry.set_password(&master_pass)?;

  let app_data_dir = &app
    .path_resolver()
    .app_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)?;

  let db_path = Path::join(&app_data_dir, "app.db");
  let path_str = db_path.to_str().unwrap();
  let connection_string = format!("sqlite:{}", path_str);
  let passphrase = generate_hex_phrase(master_pass.as_bytes())?;
  let options = ProfileDBSqliteOptions {
    password: Some(SqlitePassphrase::Hex(passphrase)),
    disable_migrate: false,
    busy_timeout: None,
  };

  let connection = ProfileDB::connect_with(&connection_string, options).await?;
  let mut db_connection = state.profile_db.write().await;
  *db_connection = Some(connection);

  Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn lock_db(
  state: State<'_, AppDbState>,
  key_state: State<'_, KeyState>,
  app: AppHandle,
) -> Result<(), AppErrors> {
  key_state.entry.delete_password()?;
  let mut db_connection = state.profile_db.write().await;
  *db_connection = None;

  let _ = app.notify_db_status();
  Ok(())
}

macro_rules! to_hex_char {
  ($val:expr) => {{
    let val: u8 = $val;

    match val {
      0..=9 => (val + 48),
      10..=15 => (val + 55),
      _ => 0,
    }
  }};
}

const PHRASE_SIZE: usize = 32usize;
const PHRASE_LOGN: u8 = 10;
const PHRASE_R: u32 = 8;
const PHRASE_P: u32 = 2;
const PHRASE_S: &[u8] = b"12345";

fn generate_hex_phrase(master_pass: &[u8]) -> Result<String, AppErrors> {
  let params = Params::new(PHRASE_LOGN, PHRASE_R, PHRASE_P, PHRASE_SIZE).map_err(|e| {
    AppErrors::LibError(String::from(
      "Passphrase generation parameters are invalid.",
    ))
  })?;

  let mut buffer: Vec<u8> = vec![0u8; PHRASE_SIZE];

  scrypt(&master_pass, PHRASE_S, &params, &mut buffer)
    .map_err(|_| AppErrors::LibError(String::from("Cannot create passphrase for sqlite.")))?;

  let mut hex_buff = vec![0u8; PHRASE_SIZE * 2];
  let mut idx: usize = 0;
  unsafe {
    for c in buffer.iter() {
      let rh = to_hex_char!(*c & 0b0000_1111);
      let lh = to_hex_char!((*c >> 4) & 0b0000_1111);

      *hex_buff.get_unchecked_mut(idx) = lh;
      idx += 1;
      *hex_buff.get_unchecked_mut(idx) = rh;
      idx += 1;
    }

    Ok(String::from_utf8_unchecked(hex_buff))
  }
}
