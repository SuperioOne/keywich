use keywich_lib::errors::Error;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

pub enum AppErrors {
  GenericError,
}

// TODO: implement this with error codes, so I can provide localization on GUI
impl From<keywich_lib::errors::Error> for AppErrors {
  fn from(value: Error) -> Self {
    AppErrors::GenericError
  }
}

impl Display for AppErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", "???")
  }
}

impl Debug for AppErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", "!!!")
  }
}

impl std::error::Error for AppErrors {}

impl From<AppErrors> for String {
  fn from(value: AppErrors) -> Self {
    match value {
      AppErrors::GenericError => String::from("Generic Error test"),
    }
  }
}

impl Serialize for AppErrors {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    // serializer.serialize_str(self.to_string().as_ref())
    let mut error_obj = serializer.serialize_struct("Error", 2)?;
    error_obj.serialize_field("code", "ERR0000")?;
    error_obj.serialize_field("message", &self.to_string())?;

    error_obj.end()
  }
}
