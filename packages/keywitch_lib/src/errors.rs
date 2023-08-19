use std::fmt::Debug;

#[derive(Debug)]
pub enum KeywitchError
{
  InvalidHashOutput,
  ParserInvalidRange,
  InvalidInput,
  InvalidConfiguration(Vec<ValidationError>),
}

#[derive(Debug)]
pub enum ValidationError
{
  EmptyCharset,
  EmptySalt,
  EmptyPassword,
  EmptyDomain,
  InvalidTargetLength,
}


