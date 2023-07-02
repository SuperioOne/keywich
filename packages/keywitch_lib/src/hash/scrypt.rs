use scrypt::{Params, scrypt};
use crate::Configuration;
use crate::errors::KeywitchError;

const SCRYPT_LOG_N: u8 = 10;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const SCRYPT_MAX_LEN: usize = 64;

pub(crate) fn hash_scrypt(options: &Configuration) -> Result<Vec<u8>, KeywitchError>
{
  // TODO: error checks
  // TODO: remove unwraps

  let params: Params = Params::new(
    SCRYPT_LOG_N,
    SCRYPT_R,
    SCRYPT_P,
    options.target_len).unwrap();

  let input = merge_vectors(options.password, options.domain);
  let mut output = vec![0u8; options.target_len];
  scrypt(&input, options.profile_salt, &params, &mut output).unwrap();

  Ok(output)
}

fn merge_vectors<T>(a: &[T], b: &[T]) -> Vec<T>
  where T: Clone
{
  let mut result = a.to_vec();
  _ = result.extend_from_slice(b);

  result
}