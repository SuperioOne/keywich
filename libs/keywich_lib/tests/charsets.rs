#[cfg(test)]
mod test {
  use keywich_lib::charset::Charset;

  #[test]
  fn valid_alpha_numeric() {
    let charset = Charset::new("abcdABCD01234").unwrap();
    assert_eq!("01234ABCDabcd", &charset.to_string());
  }

  #[test]
  fn valid_range() {
    let charset = Charset::new("a..dA..D0..4").unwrap();
    assert_eq!("01234ABCDabcd", &charset.to_string());
  }

  #[test]
  fn valid_range_with_period() {
    let charset = Charset::new("a..dA..D0..4.").unwrap();
    assert_eq!(".01234ABCDabcd", &charset.to_string());
  }

  #[test]
  #[should_panic]
  fn invalid_range_with_period() {
   Charset::new("a...dA..D0..4.").unwrap();
  }

  #[test]
  #[should_panic]
  fn invalid_range() {
   Charset::new("a...DA..D").unwrap();
  }

  #[test]
  fn duplicate_symbols() {
   let charset = Charset::new("a..da..dabcd").unwrap();
    assert_eq!("abcd", &charset.to_string());
  }
}
