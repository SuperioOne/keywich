use keywich_lib::ValidationErrors;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum AppErrors {
  OutputError(String),
  InvalidTargetLength,
  InvalidCharset,
  LibError(String),
  UnsupportedHashFunc,
  ValidationError(ValidationErrors),
  KeyNotFound,
  LocalDataDirNotFound,
  TempFolderFailed,
  ConfigPathFailed,
  LocalePathFailed,
  ContentPathFailed,
  DbNotInitialized,
  IconReadFailed(String),
  RwLockFailed,
  IconResizeFailed(String),

  KeyringFailure(String),
  NoKeyEntry,
  DuplicateKeyEntry,
}

impl AppErrors {
  pub fn code(&self) -> String {
    let code = match self {
      // User errors (1-199)
      AppErrors::OutputError(_) => 1,
      AppErrors::InvalidTargetLength => 2,
      AppErrors::InvalidCharset => 3,
      AppErrors::ValidationError(_) => 4,

      // System errors (200-399)
      AppErrors::LibError(_) => 200,
      AppErrors::UnsupportedHashFunc => 201,
      AppErrors::KeyNotFound => 202,
      AppErrors::IconReadFailed(_) => 203,
      AppErrors::IconResizeFailed(_) => 204,
      AppErrors::RwLockFailed => 205,
      AppErrors::DbNotInitialized => 206,
      AppErrors::KeyringFailure(_) => 207,
      AppErrors::NoKeyEntry => 208,
      AppErrors::DuplicateKeyEntry => 209,

      // Potential OS issues (400-599)
      AppErrors::LocalDataDirNotFound => 400,
      AppErrors::TempFolderFailed => 401,
      AppErrors::ConfigPathFailed => 402,
      AppErrors::LocalePathFailed => 403,
      AppErrors::ContentPathFailed => 404,
    };

    format!("{:0>5}", code)
  }
}

impl From<keywich_lib::errors::Error> for AppErrors {
  fn from(value: keywich_lib::errors::Error) -> Self {
    match value {
      keywich_lib::errors::Error::InvalidHashOutput => {
        AppErrors::OutputError(String::from("Invalid hash length."))
      }
      keywich_lib::errors::Error::ParserInvalidRange => AppErrors::InvalidCharset,
      keywich_lib::errors::Error::InvalidInput => AppErrors::InvalidTargetLength,
      keywich_lib::errors::Error::ValidationError(details) => AppErrors::ValidationError(details),
      keywich_lib::errors::Error::DatabaseError(detail) => AppErrors::LibError(detail),
      keywich_lib::errors::Error::DatabaseMigrateError(detail) => AppErrors::LibError(detail),
      keywich_lib::errors::Error::InvalidDatabasePath(detail) => {
        AppErrors::LibError(format!("Database not found at {:?}", detail))
      }
      keywich_lib::errors::Error::InvalidTime(detail) => AppErrors::LibError(detail),
      keywich_lib::errors::Error::InvalidHashFuncVersion => AppErrors::UnsupportedHashFunc,
      keywich_lib::errors::Error::InvalidJsonError(detail) => AppErrors::OutputError(detail),
      keywich_lib::errors::Error::InvalidQrError(detail) => AppErrors::OutputError(detail),
    }
  }
}

impl From<keyring::Error> for AppErrors {
  fn from(value: keyring::Error) -> Self {
    match value {
      keyring::Error::NoStorageAccess(_) => {
        AppErrors::KeyringFailure(String::from("Unable to access keyring."))
      }
      keyring::Error::NoEntry => AppErrors::NoKeyEntry,
      keyring::Error::BadEncoding(_) => {
        AppErrors::KeyringFailure(String::from("Keyring entry bad encoding."))
      }
      keyring::Error::TooLong(_, _) => {
        AppErrors::KeyringFailure(String::from("Keyring entry is too long."))
      }
      keyring::Error::Invalid(_, _) => {
        AppErrors::KeyringFailure(String::from("Invalid keyring entry."))
      }
      keyring::Error::Ambiguous(_) => AppErrors::DuplicateKeyEntry,
      err => AppErrors::KeyringFailure(err.to_string()),
    }
  }
}

impl Display for AppErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let message = match self {
      AppErrors::OutputError(_) => "OutputError",
      AppErrors::InvalidTargetLength => "InvalidTargetLength",
      AppErrors::InvalidCharset => "InvalidCharset",
      AppErrors::LibError(_) => "LibError",
      AppErrors::UnsupportedHashFunc => "UnsupportedHashFunc",
      AppErrors::ValidationError(_) => "ValidationError",
      AppErrors::KeyNotFound => "KeyNotFound",
      AppErrors::LocalDataDirNotFound => "LocalDataDirNotFound",
      AppErrors::TempFolderFailed => "TempFolderFailed",
      AppErrors::ConfigPathFailed => "ConfigPathFailed",
      AppErrors::LocalePathFailed => "LocalePathFailed",
      AppErrors::ContentPathFailed => "ContentPathFailed",
      AppErrors::IconReadFailed(_) => "IconReadFailed",
      AppErrors::IconResizeFailed(_) => "IconResizeFailed",
      AppErrors::DbNotInitialized => "DbNotInitialized",
      AppErrors::RwLockFailed => "RwLockFailed",
      AppErrors::KeyringFailure(_) => "KeyringFailure",
      AppErrors::NoKeyEntry => "NoKeyEntry",
      AppErrors::DuplicateKeyEntry => "DuplicateKeyEntry",
    };

    f.write_str(message)
  }
}

impl std::error::Error for AppErrors {}

macro_rules! error_obj {
  ($serializer:expr, code = $code:expr, message = $message:expr) => {{
    let c: &str = $code;
    let m: &str = $message;
    let mut error_obj = $serializer.serialize_struct("Error", 2)?;

    error_obj.serialize_field("code", c)?;
    error_obj.serialize_field("message", m)?;
    error_obj.end()
  }};
  ($serializer:expr, code = $code:expr, message = $message:expr, details = $details:expr) => {{
    let c: &str = $code;
    let m: &str = $message;
    let d: &str = $details;
    let mut error_obj = $serializer.serialize_struct("Error", 3)?;

    error_obj.serialize_field("code", c)?;
    error_obj.serialize_field("message", m)?;
    error_obj.serialize_field("details", d)?;
    error_obj.end()
  }};
  ($serializer:expr, code = $code:expr, message = $message:expr, validation = $validation:expr) => {{
    let c: &str = $code;
    let m: &str = $message;
    let v: &keywich_lib::ValidationErrors = $validation;
    let mut error_obj = $serializer.serialize_struct("Error", 3)?;

    error_obj.serialize_field("code", c)?;
    error_obj.serialize_field("message", m)?;
    error_obj.serialize_field("fields", v)?;
    error_obj.end()
  }};
}

impl Serialize for AppErrors {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      AppErrors::ValidationError(errors) => {
        error_obj!(
          serializer,
          code = &self.code(),
          message = &self.to_string(),
          validation = errors
        )
      }
      AppErrors::OutputError(details) => {
        error_obj!(
          serializer,
          code = &self.code(),
          message = &self.to_string(),
          details = details
        )
      }
      AppErrors::LibError(details) => {
        error_obj!(
          serializer,
          code = &self.code(),
          message = &self.to_string(),
          details = details
        )
      }
      AppErrors::IconReadFailed(details) => {
        error_obj!(
          serializer,
          code = &self.code(),
          message = &self.to_string(),
          details = details
        )
      }
      AppErrors::IconResizeFailed(details) => {
        error_obj!(
          serializer,
          code = &self.code(),
          message = &self.to_string(),
          details = details
        )
      }
      _ => {
        error_obj!(serializer, code = &self.code(), message = &self.to_string())
      }
    }
  }
}
