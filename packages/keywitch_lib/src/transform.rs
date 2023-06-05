use scrypt::{Params, scrypt};

const SCRYPT_LOG_N: u8 = 8;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const SCRYPT_MAX_LEN: usize = 64;

pub(crate) fn transform(input: &[u8])
{
    let params: Params = Params::new(
        SCRYPT_LOG_N,
        SCRYPT_R,
        SCRYPT_P,
        target_lenght)
        .map_err(|err| ErrMe())?;

    let mut output = vec![0u8; target_lenght];
    scrypt(&input, salt, &params, &mut output)?;
}