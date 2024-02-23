pub mod charset;
pub mod errors;
pub mod hash;

pub use scrypt;

#[cfg(feature = "profile")]
pub mod profile;

use crate::charset::{validate_charset, Charset};
use crate::errors::Error;
use crate::hash::{HashAlgorithm, HashConfig, HashGenerator};
use std::fmt;
use std::fmt::{Display, Formatter};
use validator::Validate;
pub use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Validate)]
pub struct PasswordConfig<'a> {
  #[validate(length(min = 1))]
  pub domain: &'a str,
  #[validate(length(min = 1))]
  pub password: &'a str,
  #[validate(length(min = 1))]
  pub username: &'a str,
  #[validate(length(min = 1), custom = "validate_charset")]
  pub charset: &'a str,
  pub revision: i64,
  #[validate(range(min = 1, max = 64))]
  pub target_len: usize,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct PasswordResult {
  pub pass: String,
  pub alg: String,
  pub ver: String,
}

impl PasswordResult {
  #[cfg(any(feature = "json"))]
  pub fn to_json(self) -> Result<String, Error> {
    let json_text =
      serde_json::to_string(&self).map_err(|err| Error::InvalidJsonError(err.to_string()))?;
    Ok(json_text)
  }

  #[cfg(any(feature = "base64"))]
  pub fn to_base64(self) -> String {
    use base64::Engine as _;

    base64::engine::general_purpose::STANDARD.encode(self.pass)
  }

  #[cfg(feature = "qr")]
  pub fn to_qr(self) -> Result<String, Error> {
    let bytes = self.pass.as_bytes();
    let qr_options = match bytes.len() {
      ..=16 => (qrcode::Version::Normal(2), qrcode::EcLevel::Q),
      17..=32 => (qrcode::Version::Normal(4), qrcode::EcLevel::Q),
      33..=48 => (qrcode::Version::Normal(5), qrcode::EcLevel::Q),
      _ => (qrcode::Version::Normal(6), qrcode::EcLevel::Q),
    };

    let qr = qrcode::QrCode::with_version(bytes, qr_options.0, qr_options.1)
      .map_err(|qr_err| Error::InvalidQrError(qr_err.to_string()))?;

    let qr_string = qr
      .render()
      .min_dimensions(350, 350)
      .dark_color(qrcode::render::svg::Color("#000000"))
      .light_color(qrcode::render::svg::Color("#ffffff"))
      .build();

    Ok(qr_string)
  }

  pub fn to_phc(self) -> String {
    self.to_string()
  }
}

/// Displays password result with PHC format
impl Display for PasswordResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "${alg}$v={ver}${pass}",
      alg = self.alg,
      ver = self.ver,
      pass = self.pass
    )
  }
}

pub fn generate_password(
  config: PasswordConfig,
  algorithm: HashAlgorithm,
) -> Result<PasswordResult, Error> {
  config.validate()?;

  let charset = Charset::new(config.charset)?;
  let generator = algorithm.get_generator();
  let hash = generator.generate_hash(HashConfig::from(config))?;
  let pass = charset.transform_bytes(&hash);

  Ok(PasswordResult {
    ver: generator.version().into(),
    alg: generator.name().into(),
    pass,
  })
}
