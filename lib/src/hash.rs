use crate::errors::Error;
use crate::PasswordConfig;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

mod scrypt;

use self::scrypt::KwScryptV1;

pub enum HashAlgorithm {
  // For future implementations
  KwScryptV1,
}

pub(super) trait HashGenerator {
  fn generate_hash(&self, options: HashConfig) -> Result<Vec<u8>, Error>;
  fn name(&self) -> &'static str;
  fn version(&self) -> &'static str;
}

pub(super) struct HashConfig<'a> {
  pub(super) domain: &'a [u8],
  pub(super) password: &'a [u8],
  pub(super) username: &'a [u8],
  pub(super) revision: i64,
  pub(super) target_len: usize,
}

impl HashAlgorithm {
  pub(super) fn get_generator(&self) -> impl HashGenerator {
    match self {
      HashAlgorithm::KwScryptV1 => KwScryptV1 {},
    }
  }
}

impl Display for HashAlgorithm {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      HashAlgorithm::KwScryptV1 => f.write_str("kw_scrypt:v1"),
    }
  }
}

impl Default for HashAlgorithm {
  fn default() -> Self {
    Self::KwScryptV1
  }
}

impl FromStr for HashAlgorithm {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split_once(':') {
      Some(("kw_scrypt", "v1")) => Ok(Self::KwScryptV1),
      _ => Err(Self::Err::InvalidHashFuncVersion),
    }
  }
}

impl<'a> From<PasswordConfig<'a>> for HashConfig<'a> {
  fn from(value: PasswordConfig<'a>) -> Self {
    Self {
      username: value.username.as_bytes(),
      target_len: value.target_len,
      domain: value.domain.as_bytes(),
      password: value.password.as_bytes(),
      revision: value.revision,
    }
  }
}
