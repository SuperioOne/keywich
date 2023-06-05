pub mod charset;
pub mod errors;
mod transform;

use crate::charset::{Charset};
use crate::errors::KeywitchError;
use scrypt::Params;
use scrypt::errors::InvalidOutputLen;
use scrypt::{scrypt};
use std::fmt::Error;
use crate::transform::transform;


pub fn calculate_password(
    domain: &[u8],
    password: &[u8],
    salt: &[u8],
    target_lenght: usize,
    // charset: Charset)
) -> Result<(), KeywitchError>
{
    let separator_idx = password.len();
    let size = separator_idx + domain.len();
    let mut input: Vec<u8> = vec![0u8; size];

    let password_part = &mut input[0..separator_idx];
    password_part.copy_from_slice(password);

    let domain_part = &mut input[separator_idx..];
    domain_part.copy_from_slice(domain);

    let params: Params = Params::new(
        SCRYPT_LOG_N,
        SCRYPT_R,
        SCRYPT_P,
        target_lenght)
        .map_err(|err| ErrMe())?;

    let mut output = vec![0u8; target_lenght];
    scrypt(&input, salt, &params, &mut output).map_err(|err| ErrMe())?;

    Ok(())
}


