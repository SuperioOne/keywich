#[cfg(test)]
mod test {
  use keywich_lib::errors::Error;
  use keywich_lib::hash::HashAlgorithm;
  use keywich_lib::{generate_password, PasswordConfig};

  #[test]
  fn kwscrypt_basic_password_test() {
    let config = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let result = generate_password(config, HashAlgorithm::KwScryptV1).unwrap();

    assert_eq!("crofpiqowbnl", &result.pass);
    assert_eq!(12, result.pass.len());
  }

  #[test]
  fn invalid_config_test() {
    let config = PasswordConfig {
      charset: "",
      target_len: 65,
      password: "",
      username: "",
      domain: "",
      revision: 1,
    };

    if let Err(Error::InvalidConfiguration(validation_errors)) =
      generate_password(config, HashAlgorithm::KwScryptV1)
    {
      assert_eq!(5, validation_errors.len())
    } else {
      assert!(false);
    }
  }

  #[test]
  fn kwscrypt_revision_test() {
    let config_a = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let config_b = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 2,
    };

    let result_a = generate_password(config_a, HashAlgorithm::KwScryptV1).unwrap();
    let result_b = generate_password(config_b, HashAlgorithm::KwScryptV1).unwrap();

    assert_eq!("crofpiqowbnl", &result_a.pass);
    assert_eq!("cuudxgkeiihw", &result_b.pass);
  }

  #[test]
  fn kwscrypt_short_password_test() {
    let config = PasswordConfig {
      charset: "a..z",
      target_len: 4,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let result = generate_password(config, HashAlgorithm::KwScryptV1).unwrap();

    assert_eq!(4, result.pass.len());
    assert_eq!("decx", &result.pass);
  }

  #[cfg(feature = "base64")]
  #[test]
  fn kwscrypt_base64_test() {
    let config = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let pass = generate_password(config, HashAlgorithm::KwScryptV1).unwrap();
    let base64_string = pass.to_base64().unwrap();

    assert_eq!("Y3JvZnBpcW93Ym5s", &base64_string);
  }

  #[cfg(feature = "json")]
  #[test]
  fn kwscrypt_json_test() {
    let config = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let pass = generate_password(config, HashAlgorithm::KwScryptV1).unwrap();
    let json_text = pass.to_json();

    assert!(json_text.is_ok());
  }

  #[cfg(feature = "qr")]
  #[test]
  fn kwscrypt_qr_test() {
    let config = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let pass = generate_password(config, HashAlgorithm::KwScryptV1).unwrap();
    let qr = pass.to_qr();

    assert!(qr.is_ok());
  }

  #[test]
  fn kwscrypt_phc_test() {
    let config = PasswordConfig {
      charset: "a..z",
      target_len: 12,
      password: "test",
      username: "john",
      domain: "acme",
      revision: 1,
    };

    let pass = generate_password(config, HashAlgorithm::KwScryptV1).unwrap();
    let phc_text = pass.to_phc();

    assert_eq!("$kw_scrypt$v=v1$crofpiqowbnl", &phc_text);
  }
}
