use crate::Configuration;

mod scrypt_transformer;

use scrypt_transformer::{transform};

pub enum PasswordAlgo
{
  ScryptFunc
}

pub trait PasswordFunc
{
  fn generate_hash(options: &Configuration);
}

impl PasswordFunc for PasswordAlgo
{
  fn generate_hash(options: &Configuration) {
    transform(options)
  }
}