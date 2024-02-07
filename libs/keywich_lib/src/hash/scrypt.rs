use crate::errors::Error;
use crate::hash::{HashConfig, HashGenerator};
use bytes::{BufMut, BytesMut};
use scrypt::errors::{InvalidOutputLen, InvalidParams};
use scrypt::{scrypt, Params};
use std::io::Read;

const SCRYPT_LOG_N: u8 = 10;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const SCRYPT_MIN_LEN: usize = 10;

pub(super) struct KwScryptV1 {}

impl HashGenerator for KwScryptV1 {
  fn generate_hash(&self, options: HashConfig) -> Result<Vec<u8>, Error> {
    let mut fold = false;
    let scrypt_target_len = {
      if options.target_len < SCRYPT_MIN_LEN {
        fold = true;
        SCRYPT_MIN_LEN
      } else {
        options.target_len
      }
    };

    let mut byte_buffer = BytesMut::from(options.username);
    byte_buffer.put_u32_le('@'.into());
    byte_buffer.put_slice(&options.domain.to_ascii_lowercase());
    byte_buffer.put_u32_le('r'.into());
    byte_buffer.put_i64_le(options.revision);

    let params: Params = Params::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, scrypt_target_len)?;
    let mut output = vec![0u8; scrypt_target_len];
    scrypt(options.password, &byte_buffer, &params, &mut output)?;

    if fold {
      Ok(fold_content(&output, options.target_len))
    } else {
      Ok(output)
    }
  }

  fn name(&self) -> &'static str {
    "kw_scrypt"
  }

  fn version(&self) -> &'static str {
    "v1"
  }
}

#[inline]
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

impl From<InvalidOutputLen> for Error {
  fn from(_value: InvalidOutputLen) -> Self {
    Self::InvalidHashOutput
  }
}

impl From<InvalidParams> for Error {
  fn from(_value: InvalidParams) -> Self {
    Self::InvalidInput
  }
}
