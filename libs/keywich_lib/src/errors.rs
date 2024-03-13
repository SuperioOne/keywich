use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
  InvalidHashOutput,
  ParserInvalidRange,
  InvalidInput,
  DatabaseError(String),
  DatabaseMigrateError(String),
  InvalidDatabasePath(PathBuf),
  InvalidTime(String),
  InvalidHashFuncVersion,
  InvalidJsonError(String),
  InvalidQrError(String),
  ValidationError(validator::ValidationErrors),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::InvalidHashOutput => write!(f, "Generated input output length is not valid."),
      Error::ParserInvalidRange => write!(f, "Charset range is not valid."),
      Error::InvalidInput => write!(f, "Hash input options are not valid."),
      Error::DatabaseError(err) => write!(f, "Database action failed. Reason: {}", err),
      Error::DatabaseMigrateError(err) => write!(f, "Database migration failed. Reason: {}", err),
      Error::InvalidDatabasePath(path) => write!(f, "Database path {:?} is not accessible.", path),
      Error::InvalidTime(err) => write!(f, "Unix timestamp input cannot be parsed. {}", err),
      Error::InvalidHashFuncVersion => write!(f, "Unsupported hash function version received"),
      Error::InvalidJsonError(err) => {
        write!(f, "Password json serialization failed. Reason: {}", err)
      }
      Error::InvalidQrError(err) => write!(f, "Password qr generation failed. Reason: {}", err),
      Error::ValidationError(err) => write!(f, "Input validation failed, {}", err),
    }
  }
}

impl From<validator::ValidationErrors> for Error {
  fn from(value: validator::ValidationErrors) -> Self {
    Self::ValidationError(value)
  }
}

#[cfg(feature = "profile")]
impl From<sqlx::Error> for Error {
  fn from(value: sqlx::Error) -> Self {
    match value {
      sqlx::Error::RowNotFound => Error::DatabaseError("Row not found".into()),
      sqlx::Error::TypeNotFound { type_name } => {
        Error::DatabaseError(format!("type {} not found.", type_name))
      }
      sqlx::Error::ColumnIndexOutOfBounds { index, len } => Error::DatabaseError(format!(
        "Column index out of bounds. index {} len {}",
        index, len
      )),
      sqlx::Error::ColumnDecode { index, source } => Error::DatabaseError(format!(
        "Column decode failed. source {}, index {}",
        source, index
      )),
      sqlx::Error::PoolTimedOut => Error::DatabaseError("Pool timed out.".into()),
      sqlx::Error::PoolClosed => Error::DatabaseError("Pool closed.".into()),
      sqlx::Error::WorkerCrashed => Error::DatabaseError("Worker crashed.".into()),
      err => Error::DatabaseError(err.to_string()),
    }
  }
}

#[cfg(feature = "profile")]
impl From<sqlx::migrate::MigrateError> for Error {
  fn from(value: sqlx::migrate::MigrateError) -> Self {
    Error::DatabaseMigrateError(value.to_string())
  }
}
