use crate::errors::AppErrors;
use crate::{AppDbState, DbNotifier, KeyState, PasswordOutputType};
use keywich_lib::hash::HashAlgorithm;
use serde::Deserialize;
use std::ops::Deref;
use std::str::FromStr;
use tauri::{AppHandle, State};

#[derive(Deserialize)]
pub struct PasswordRequest {
  pub profile_id: i64,
  pub output_type: PasswordOutputType,
}

#[derive(Deserialize)]
pub struct PasswordGenerateRequest {
  pub content: String,
  pub revision: i64,
  pub domain: String,
  pub username: String,
  pub charset: String,
  pub version: String,
  pub target_len: i64,
  pub output_type: PasswordOutputType,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_password_from(
  state: State<'_, AppDbState>,
  key_state: State<'_, KeyState>,
  app: AppHandle,
  request: PasswordRequest,
) -> Result<String, AppErrors> {
  let PasswordRequest {
    profile_id,
    output_type,
  } = request;

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

  let read_lock = state.profile_db.read().await;

  if let Some(profile_db) = read_lock.deref() {
    if let Some(key) = profile_db.get_key_by_id(profile_id).await? {
      let target_len =
        usize::try_from(key.target_size).map_err(|_err| AppErrors::InvalidTargetLength)?;
      let config = keywich_lib::PasswordConfig {
        password: &password,
        revision: key.revision,
        domain: &key.domain,
        username: &key.username,
        charset: &key.charset,
        target_len,
      };

      generate(config, output_type, Some(&key.version))
    } else {
      Err(AppErrors::KeyNotFound)
    }
  } else {
    let _ = app.emit_unlock_required();
    Err(AppErrors::DbNotInitialized)
  }
}

#[tauri::command(rename_all = "snake_case")]
pub fn generate_password(request: PasswordGenerateRequest) -> Result<String, AppErrors> {
  let PasswordGenerateRequest {
    content,
    version,
    output_type,
    charset,
    username,
    domain,
    revision,
    target_len,
  } = request;

  let target_len = usize::try_from(target_len).map_err(|_err| AppErrors::InvalidTargetLength)?;

  let config = keywich_lib::PasswordConfig {
    charset: &charset,
    domain: &domain,
    password: &content,
    username: &username,
    revision,
    target_len,
  };

  generate(config, output_type, Some(&version))
}

pub(crate) fn generate(
  config: keywich_lib::PasswordConfig,
  output_type: PasswordOutputType,
  algo: Option<&str>,
) -> Result<String, AppErrors> {
  let hash_algo = match algo {
    None => HashAlgorithm::default(),
    Some(algo_name) => HashAlgorithm::from_str(algo_name)?,
  };
  let pass_result = keywich_lib::generate_password(config, hash_algo)?;
  let string_response = match output_type {
    PasswordOutputType::PHC => pass_result.to_phc(),
    PasswordOutputType::Text => pass_result.pass,
    PasswordOutputType::Base64 => pass_result.to_base64(),
    PasswordOutputType::Json => pass_result.to_json()?,
    PasswordOutputType::Qr => pass_result.to_qr()?,
  };

  Ok(string_response)
}
