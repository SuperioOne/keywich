use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
  InvalidHashOutput,
  ParserInvalidRange,
  InvalidInput,
  InvalidConfiguration(Vec<ValidationError>),
  DatabaseError(String),
  DatabaseMigrateError(String),
  InvalidDatabasePath(PathBuf),
  InvalidTime(String),
  InvalidHashFuncVersion,
  InvalidJsonError(String),
  InvalidQrError(String),
}

#[derive(Debug)]
pub enum ValidationError {
  EmptyCharset,
  EmptySalt,
  EmptyPassword,
  EmptyDomain,
  InvalidTargetLength,
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
