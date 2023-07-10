use scrypt::{Params, scrypt};
use crate::Configuration;
use crate::errors::{KeywitchError};

const SCRYPT_LOG_N: u8 = 10;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const SCRYPT_MAX_LEN: usize = 64;
const SCRYPT_MIN_LEN: usize = 10;

pub(crate) fn hash_scrypt(options: &Configuration) -> Result<Vec<u8>, KeywitchError>
{
  // TODO: loggers

  let mut truncate = false;
  let target_len = {
    if options.target_len < SCRYPT_MIN_LEN
    {
      truncate = true;
      10_usize
    } else { options.target_len }
  };

  let params: Params = Params::new(
    SCRYPT_LOG_N,
    SCRYPT_R,
    SCRYPT_P,
    target_len)
    .map_err(|err| {
      println!("{:?}", err);
      KeywitchError::InvalidInput
    })?;

  let input = merge_vectors(options.password, options.domain);
  let mut output = vec![0u8; options.target_len];
  scrypt(&input, options.profile_salt, &params, &mut output)
    .map_err(|err| {
      println!("{:?}", err);
      KeywitchError::InvalidHashOutput
    })?;

  if truncate
  {
    // Maybe we should do some folding instead of truncating
    let truncated = (&output[0..options.target_len]).to_owned();
    Ok(truncated)
  } else {
    Ok(output)
  }
}

fn merge_vectors<T>(a: &[T], b: &[T]) -> Vec<T>
  where T: Clone
{
  let mut result = a.to_vec();
  _ = result.extend_from_slice(b);

  result
}