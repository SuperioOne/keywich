#[cfg(test)]
mod tests {
  use keywitch_lib::{generate_password, charset, Configuration};
  use scrypt::Params;
  use scrypt::{scrypt};
  use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

  #[test]
  fn it_works() {
    // let dictionary = "a-zA-Z@:/\\£5^&0-9\\-";

    let parsed = charset::parser::parse("a..zA..Z0..9-_@,.@&*+!\"'");
    println!("{:?}", parsed);

    let charset = charset::Charset {
      charset: parsed.unwrap()
    };

    let p_pass = String::from("SuperSecretPassword");
    let p_domain = String::from("x");
    let p_salt = String::from("SuperiorOne");

    let config = Configuration {
      charset: charset,
      password: p_pass.as_bytes(),
      domain: p_domain.as_bytes(),
      profile_salt: p_salt.as_bytes(),
      target_len: 5,
    };

    let pass = generate_password(&config).unwrap();

    println!("ver: {}, alg: {}, pass: {}", pass.ver, pass.alg, pass.pass);
    println!("{}", pass);
  }
}
