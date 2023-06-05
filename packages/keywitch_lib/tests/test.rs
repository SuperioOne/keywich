#[cfg(test)]
mod tests {
    use std::io::Read;
    use scrypt::Params;
    use scrypt::{scrypt};
    use super::*;
    use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
    use keywitch_lib::calculate_password;

    #[test]
    fn it_works() {
        let pass = b"1234";
        let salt = b"salt";
        let dmn = b"dmn";
        calculate_password(dmn ,pass, salt, 32_usize);
    }

    #[test]
    fn benchmark()
    {
        let pass = b"12345678901112aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaasdddssssssssssssssssssssssssssssssssssaa";
        let salt = b"salt";
        let params: Params = Params::new(8, 8, 1, 64).unwrap();
        let mut output: Vec<u8> = vec![0u8; 64];

        scrypt(pass, salt, &params, output.as_mut_slice()).unwrap();

        let mut buf = String::new();
        general_purpose::STANDARD.encode_string(&output, &mut buf);
        println!("{}", buf);

        println!("{:?}", buf);
    }
}
