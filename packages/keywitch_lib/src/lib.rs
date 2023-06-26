pub mod charset;
pub mod errors;
mod hash;

use crate::charset::Charset;
use crate::errors::KeywitchError;

pub struct Configuration<'a>
{
  pub domain: &'a [u8],
  pub password: &'a [u8],
  pub profile_salt: &'a [u8],
  pub charset: &'a Charset,
  pub target_len: usize,
}

pub fn generate_password(options: &Configuration) -> Result<(), KeywitchError>
{
  Ok(())
}
