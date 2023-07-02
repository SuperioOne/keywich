use crate::Configuration;
use crate::errors::KeywitchError;

mod scrypt;

use self::scrypt::{hash_scrypt};

pub enum PasswordAlgo
{
  ScryptV1
}

impl PasswordAlgo
{
  pub fn generate_hash(&self, options: &Configuration) -> Result<Vec<u8>, KeywitchError> {
    match self {
      PasswordAlgo::ScryptV1 => hash_scrypt(options),
    }
  }

  pub fn name(&self) -> String
  {
    match self {
      PasswordAlgo::ScryptV1 => "scrypt".into(),
    }
  }

  pub fn version(&self) -> String
  {
    match self {
      PasswordAlgo::ScryptV1 => "v1".into(),
    }
  }
}