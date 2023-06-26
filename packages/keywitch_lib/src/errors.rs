use core::result::Result;
use std::error::Error;

#[derive(Debug)]
pub enum KeywitchError
{
  EmptyInput,
  InvalidPasswordLength,
  InvalidInput,
}

pub trait CustomErrorExt<T>
{
  fn map_keywitch_err(&self) -> Result<T, KeywitchError>;
}

impl<T, E> CustomErrorExt<T> for Result<T, E> where E: Error
{
  fn map_keywitch_err(&self) -> Result<T, KeywitchError> {
    todo!()
  }
}