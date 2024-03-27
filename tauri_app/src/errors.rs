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
  IconResizeFailed(String),
  KeyringFailure(String),
  NoKeyEntry,
  DuplicateKeyEntry,
  BackupError(String),
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
      AppErrors::DbNotInitialized => 206,
      AppErrors::KeyringFailure(_) => 207,
      AppErrors::NoKeyEntry => 208,
      AppErrors::DuplicateKeyEntry => 209,
      AppErrors::BackupError(_) => 210,

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
        Self::OutputError(String::from("Invalid hash length."))
      }
      keywich_lib::errors::Error::ParserInvalidRange => Self::InvalidCharset,
      keywich_lib::errors::Error::InvalidInput => Self::InvalidTargetLength,
      keywich_lib::errors::Error::ValidationError(details) => Self::ValidationError(details),
      keywich_lib::errors::Error::DatabaseError(detail) => Self::LibError(detail),
      keywich_lib::errors::Error::DatabaseMigrateError(detail) => Self::LibError(detail),
      keywich_lib::errors::Error::InvalidDatabasePath(detail) => {
        Self::LibError(format!("Database not found at {:?}", detail))
      }
      keywich_lib::errors::Error::InvalidTime(detail) => Self::LibError(detail),
      keywich_lib::errors::Error::InvalidHashFuncVersion => Self::UnsupportedHashFunc,
      keywich_lib::errors::Error::InvalidJsonError(detail) => Self::OutputError(detail),
      keywich_lib::errors::Error::InvalidQrError(detail) => Self::OutputError(detail),
      keywich_lib::errors::Error::BackupError(detail) => Self::BackupError(detail),
    }
  }
}

impl From<keyring::Error> for AppErrors {
  fn from(value: keyring::Error) -> Self {
    match value {
      keyring::Error::NoStorageAccess(_) => {
        Self::KeyringFailure(String::from("Unable to access keyring."))
      }
      keyring::Error::NoEntry => Self::NoKeyEntry,
      keyring::Error::BadEncoding(_) => {
        Self::KeyringFailure(String::from("Keyring entry bad encoding."))
      }
      keyring::Error::TooLong(_, _) => {
        Self::KeyringFailure(String::from("Keyring entry is too long."))
      }
      keyring::Error::Invalid(_, _) => Self::KeyringFailure(String::from("Invalid keyring entry.")),
      keyring::Error::Ambiguous(_) => Self::DuplicateKeyEntry,
      err => Self::KeyringFailure(err.to_string()),
    }
  }
}

impl Display for AppErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      AppErrors::OutputError(err) => write!(f, "Password output generation failed. {}", err),
      AppErrors::InvalidTargetLength => write!(f, "Password target length is not valid."),
      AppErrors::InvalidCharset => write!(f, "Provided charset syntax is not valid."),
      AppErrors::LibError(err) => write!(f, "Unexpected error, {}", err),
      AppErrors::UnsupportedHashFunc => write!(f, "Unsupported hash function received."),
      AppErrors::ValidationError(err) => write!(f, "Input validation failed {}", err),
      AppErrors::KeyNotFound => write!(f, "Requested key does not exists."),
      AppErrors::LocalDataDirNotFound => write!(f, "Local data directory not found."),
      AppErrors::TempFolderFailed => write!(f, "Unable to access app temp directory."),
      AppErrors::ConfigPathFailed => write!(f, "Unable to access app config file."),
      AppErrors::LocalePathFailed => write!(f, "Unable to access app locale directory."),
      AppErrors::ContentPathFailed => write!(f, "Unable to access app content directory."),
      AppErrors::IconReadFailed(err) => write!(f, "Cannot read key icon content, {}", err),
      AppErrors::IconResizeFailed(err) => write!(f, "Icon resize operation failed, {}", err),
      AppErrors::DbNotInitialized => write!(f, "Database is not initialized."),
      AppErrors::KeyringFailure(err) => write!(f, "OS keyring failed, {}", err),
      AppErrors::NoKeyEntry => write!(f, "No master key entry found."),
      AppErrors::DuplicateKeyEntry => write!(f, "Duplicate master key entry detected."),
      AppErrors::BackupError(err) => write!(f, "Backup action failed, {}", err),
    }
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
      AppErrors::BackupError(details) => {
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
