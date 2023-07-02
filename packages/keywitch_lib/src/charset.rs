pub struct Charset
{
  charset: Vec<u8>,
  modulus: usize,
}

impl Charset
{
  pub(crate) fn transform_str(&self, content: &str) -> String
  {
    self.transform_bytes(content.as_bytes())
  }

  pub(crate) fn transform_bytes(&self, content: &[u8]) -> String
  {
    let mapped: Vec<u8> = content
      .iter()
      .map(|item| {
        let lookup_idx = (*item) as usize % self.modulus;
        unsafe {
          let lookup = self.charset.get_unchecked(lookup_idx);
          *lookup
        }
      })
      .collect();

    // todo handle possible utf-8 error
    let str = String::from_utf8(mapped).unwrap();

    str
  }
}


