use crate::errors::Error;
use std::num::TryFromIntError;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

/// Returns a Unix timestamp compatible with Sqlite INT(i64).
pub(crate) fn get_unix_timestamp() -> Result<i64, Error> {
  let now: i64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)?
    .as_secs()
    .try_into()?;

  Ok(now)
}

impl From<SystemTimeError> for Error {
  fn from(value: SystemTimeError) -> Self {
    Error::InvalidTime(value.to_string())
  }
}

impl From<TryFromIntError> for Error {
  fn from(value: TryFromIntError) -> Self {
    Error::InvalidTime(value.to_string())
  }
}
