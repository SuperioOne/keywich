pub mod charset;
pub mod errors;
mod hash;
pub mod profile;

use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize};
use std::fmt::{Display, Formatter};
use std::fmt;
use qrcode::{EcLevel, QrCode, Version};
use qrcode::render::svg;
use crate::charset::Charset;
use crate::errors::{KeywitchError, ValidationError};
use crate::hash::{PasswordAlgo};

pub struct Configuration<'a>
{
  pub domain: &'a [u8],
  pub password: &'a [u8],
  pub profile_salt: &'a [u8],
  pub charset: Charset,
  pub target_len: usize,
}

#[derive(Serialize, Debug)]
pub struct PasswordResult
{
  pub pass: String,
  pub alg: String,
  pub ver: String,
}

pub enum OutputType
{
  PasswordText,
  Text,
  Base64,
  Json,
  QR,
}

impl PasswordResult
{
  pub fn to_formatted_string(&self, output_type: OutputType) -> String
  {
    match output_type {
      OutputType::Text => self.pass.clone(),
      OutputType::Json => serde_json::to_string(self).unwrap_or(String::from("Invalid JSON")),
      OutputType::Base64 => general_purpose::STANDARD_NO_PAD.encode(&self.pass),
      OutputType::QR => {
        let qr = QrCode::with_version(self.pass.as_bytes(), Version::Normal(3), EcLevel::Q).unwrap();
        qr.render()
          .min_dimensions(350, 350)
          .dark_color(svg::Color("#000000"))
          .light_color(svg::Color("#ffffff"))
          .build()
      }
      _ => self.to_string(),
    }
  }
}

impl Display for PasswordResult
{
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f,
           "${alg}${ver}${pass}",
           alg = self.alg,
           ver = self.ver,
           pass = self.pass)
  }
}

impl<'a> Configuration<'a>
{
  pub fn new(
    domain: &'a str,
    pass: &'a str,
    salt: &'a str,
    charset_text: &'a str,
    target_len: usize) -> Result<Self, KeywitchError>
  {
    let mut errors: Vec<ValidationError> = vec!();

    if target_len < 1 || target_len > 64 {
      errors.push(ValidationError::InvalidTargetLength);
    }

    if domain.trim().is_empty() {
      errors.push(ValidationError::EmptyDomain);
    }

    if pass.trim().is_empty() {
      errors.push(ValidationError::EmptyPassword);
    }

    if salt.trim().is_empty() {
      errors.push(ValidationError::EmptySalt);
    }

    if charset_text.is_empty() {
      errors.push(ValidationError::EmptyCharset);
    }

    if errors.is_empty() == false {
      return Err(KeywitchError::InvalidConfiguration(errors));
    }

    let charset = Charset::new(charset_text)?;
    Ok(Configuration {
      target_len,
      password: pass.as_bytes(),
      profile_salt: salt.as_bytes(),
      domain: domain.as_bytes(),
      charset,
    })
  }
}

pub fn generate_password(config: &Configuration) -> Result<PasswordResult, KeywitchError>
{
  let hash = PasswordAlgo::ScryptV1.generate_hash(config)?;
  let pass = config.charset.transform_bytes(&hash);

  Ok(
    PasswordResult {
      ver: PasswordAlgo::ScryptV1.version(),
      alg: PasswordAlgo::ScryptV1.name(),
      pass,
    }
  )
}
