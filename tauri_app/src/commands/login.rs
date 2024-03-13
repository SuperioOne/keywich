use crate::errors::AppErrors;
use crate::result_log::ResultLog;
use crate::{AppDbState, DbNotifier, KeyState};
use keywich_lib::charset::Charset;
use keywich_lib::profile::{ProfileDB, ProfileDBSqliteOptions, SqlitePassphrase};
use keywich_lib::scrypt::{scrypt, Params};
use log::{debug, error, info, warn};
use std::path::Path;
use tauri::{AppHandle, State};

pub(super) const APP_DB_NAME: &str = "app.db";

#[tauri::command(rename_all = "snake_case")]
pub async fn unlock_db(
  state: State<'_, AppDbState>,
  key_state: State<'_, KeyState>,
  app: AppHandle,
  master_pass: String,
) -> Result<(), AppErrors> {
  key_state.entry.set_password(&master_pass).log_err()?;

  let local_data_dir = &app
    .path_resolver()
    .app_local_data_dir()
    .ok_or(AppErrors::LocalDataDirNotFound)
    .log_err()?;

  let db_path = Path::join(local_data_dir, APP_DB_NAME);
  let path_str = db_path
    .to_str()
    .ok_or(AppErrors::DbNotInitialized)
    .log_err()?;

  let connection_string = format!("sqlite:{}", path_str);
  let connection = match login(&connection_string, &master_pass).await {
    Ok(profile_db) => {
      info!("Connected with v0.2.0 passphrase generator.");
      profile_db
    }
    Err(e) => {
      warn!("Connected with v0.2.0 failed trying with v0.1.0");
      debug!("{}", e);
      let db = login_v0_1_0(&connection_string, &master_pass)
        .await
        .log_err()?;

      warn!("Connection with v0.1.0 succeed and passphrase migrated to v0.2.0");

      db
    }
  };

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

  let _ = app.emit_unlock_required();
  Ok(())
}

async fn login(conn_str: &str, master_pass: &str) -> Result<ProfileDB, AppErrors> {
  let passphrase = generate_phrase(master_pass.as_bytes()).log_err()?;
  let options = ProfileDBSqliteOptions {
    password: Some(SqlitePassphrase::Text(passphrase)),
    new_password: None,
    disable_migrate: false,
    busy_timeout: None,
  };
  let connection = ProfileDB::connect_with(conn_str, options).await.log_err()?;

  Ok(connection)
}

/// DEPRECATED - Tries to unlock app db with old passphrase generator and migrates to the newer one.
/// # Arguments
///
/// * `conn_str`: Database connection string
/// * `master_pass`: Master password
///
/// returns: Result<ProfileDB, AppErrors>
#[deprecated]
async fn login_v0_1_0(conn_str: &str, master_pass: &str) -> Result<ProfileDB, AppErrors> {
  let passphrase = generate_hex_phrase(master_pass.as_bytes()).log_err()?;
  let new_passphrase = generate_phrase(master_pass.as_bytes()).log_err()?;
  let options = ProfileDBSqliteOptions {
    password: Some(SqlitePassphrase::Hex(passphrase)),
    new_password: Some(SqlitePassphrase::Text(new_passphrase)),
    disable_migrate: false,
    busy_timeout: None,
  };
  let connection = ProfileDB::connect_with(conn_str, options).await.log_err()?;

  Ok(connection)
}

const PHRASE_SIZE: usize = 32usize;
const PHRASE_LOG_N: u8 = 10;
const PHRASE_R: u32 = 8;
const PHRASE_P: u32 = 2;
const PHRASE_S: &[u8] = b"12345";

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

macro_rules! hex_string {
  ($content:expr) => {{
    let input: &[u8] = $content;
    let mut buffer: Vec<u8> = vec![0u8; input.len() * 2];
    let mut idx: usize = 0;

    unsafe {
      for c in input.iter() {
        let rh = to_hex_char!(*c & 0b0000_1111);
        let lh = to_hex_char!((*c >> 4) & 0b0000_1111);

        *buffer.get_unchecked_mut(idx) = lh;
        idx += 1;
        *buffer.get_unchecked_mut(idx) = rh;
        idx += 1;
      }

      String::from_utf8_unchecked(buffer)
    }
  }};
}

#[deprecated]
fn generate_hex_phrase(master_pass: &[u8]) -> Result<String, AppErrors> {
  let params = Params::new(PHRASE_LOG_N, PHRASE_R, PHRASE_P, 32).map_err(|e| {
    error!("Passphrase parameter error {}", e);
    AppErrors::LibError(String::from(
      "Passphrase generation parameters are invalid.",
    ))
  })?;

  let mut buffer: Vec<u8> = vec![0u8; PHRASE_SIZE];

  scrypt(master_pass, PHRASE_S, &params, &mut buffer).map_err(|e| {
    error!("Passphrase generator error {}", e);
    AppErrors::LibError(String::from("Cannot create passphrase for sqlite."))
  })?;

  Ok(hex_string!(&buffer))
}

fn generate_phrase(master_pass: &[u8]) -> Result<String, AppErrors> {
  let params = Params::new(PHRASE_LOG_N, PHRASE_R, PHRASE_P, 64)
    .map_err(|e| {
      error!("Passphrase parameter error {}", e);
      AppErrors::LibError(String::from(
        "Passphrase generation parameters are invalid.",
      ))
    })
    .log_err()?;

  let charset = Charset::new("a..zA..Z0..9").log_err()?;
  let mut buffer: Vec<u8> = vec![0u8; PHRASE_SIZE];

  scrypt(master_pass, PHRASE_S, &params, &mut buffer).map_err(|e| {
    error!("Passphrase generator error {}", e);
    AppErrors::LibError(String::from("Cannot create passphrase for sqlite."))
  })?;

  Ok(charset.transform_bytes(&buffer))
}
