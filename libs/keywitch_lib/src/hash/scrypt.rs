use crate::errors::Error;
use crate::Configuration;
use scrypt::{scrypt, Params};

const SCRYPT_LOG_N: u8 = 10;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const SCRYPT_MAX_LEN: usize = 64;
const SCRYPT_MIN_LEN: usize = 10;

pub(crate) fn hash_scrypt(options: &Configuration) -> Result<Vec<u8>, Error> {
  // TODO: loggers

  let mut fold = false;
  let scrypt_target_len = {
    if options.target_len < SCRYPT_MIN_LEN {
      fold = true;
      10_usize
    } else {
      options.target_len
    }
  };

  let params: Params = Params::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, scrypt_target_len)
    .map_err(|err| Error::InvalidInput)?;

  let input = merge_vectors(options.password, options.domain);
  let mut output = vec![0u8; scrypt_target_len];
  scrypt(&input, options.profile_salt, &params, &mut output)
    .map_err(|err| Error::InvalidHashOutput)?;

  if fold {
    Ok(fold_content(&output, options.target_len))
  } else {
    Ok(output)
  }
}

fn fold_content(content: &[u8], target_len: usize) -> Vec<u8> {
  let mut response_content = content[..target_len].to_vec();
  let extra_content = &content[target_len..];

  for (index, value) in extra_content.iter().enumerate() {
    unsafe {
      let element = response_content.get_unchecked_mut(index % target_len);
      *element ^= *value;
    }
  }

  response_content
}

fn merge_vectors<T>(a: &[T], b: &[T]) -> Vec<T>
where
  T: Clone,
{
  let mut result = a.to_vec();
  _ = result.extend_from_slice(b);

  result
}
