pub mod charset;
pub mod errors;
mod hash;

use std::fmt;
use std::fmt::{Display, Formatter};
use crate::charset::Charset;
use crate::errors::KeywitchError;
use crate::hash::{PasswordAlgo};

pub struct Configuration<'a>
{
  pub domain: &'a [u8],
  pub password: &'a [u8],
  pub profile_salt: &'a [u8],
  pub charset: &'a Charset,
  pub target_len: usize,
}

pub struct PasswordResult
{
  pub pass: String,
  pub alg: String,
  pub ver: String,
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

pub fn generate_password(config: &Configuration) -> Result<PasswordResult, KeywitchError>
{
  // TODO: validate configuration

  let hash = PasswordAlgo::ScryptV1.generate_hash(config)?;
  let pass = config.charset.transform_bytes(&hash);

  Ok(
    PasswordResult {
      ver: PasswordAlgo::ScryptV1.version(),
      alg: PasswordAlgo::ScryptV1.name(),
      pass: pass,
    }
  )
}
