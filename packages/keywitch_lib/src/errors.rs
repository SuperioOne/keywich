use core::result::Result;

#[derive(Debug)]
pub enum KeywitchError
{
    EmptyInput,
    InvalidPasswordLength,
    InvalidInput,
}

pub trait CustomErrorExt
{
    fn map_keywitch_err<T>(&self) -> Result<T, KeywitchError>;
}

impl<T, E> CustomErrorExt for Result<T, E>
{
    fn map_keywitch_err<T>(&self) -> Result<T, KeywitchError> {
        todo!()
    }
}