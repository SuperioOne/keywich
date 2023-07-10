use std::fmt::Debug;

#[derive(Debug)]
pub enum KeywitchError
{
  EmptyInput,
  InvalidPasswordLength,
  InvalidInput,
  InvalidHashOutput,
}
