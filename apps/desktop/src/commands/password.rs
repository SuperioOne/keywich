use crate::errors::AppErrors;
use crate::{AppRpcState, PasswordOutputType};
use keywich_lib::hash::HashAlgorithm;
use serde::Deserialize;
use std::str::FromStr;
use tauri::State;

#[derive(Deserialize)]
pub struct PasswordRequest {
  pub profile_id: i64,
  pub content: String,
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

#[tauri::command]
pub async fn generate_password_from(
  state: State<'_, AppRpcState>,
  request: PasswordRequest,
) -> Result<String, AppErrors> {
  let PasswordRequest {
    content,
    profile_id,
    output_type,
  } = request;

  if let Some(key) = state.profile_db.get_key_by_id(profile_id).await? {
    let target_len =
      usize::try_from(key.target_size).map_err(|_err| keywich_lib::errors::Error::InvalidInput)?;
    let config = keywich_lib::PasswordConfig {
      password: &content,
      revision: key.revision,
      domain: &key.domain,
      username: &key.username,
      charset: &key.charset,
      target_len,
    };

    generate(config, output_type, Some(&key.version))
  } else {
    Err(AppErrors::GenericError)
  }
}

#[tauri::command]
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

  let target_len =
    usize::try_from(target_len).map_err(|_err| keywich_lib::errors::Error::InvalidInput)?;

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
