use crate::charset::parser::parse;
use crate::errors::Error;
use std::fmt::{Display, Formatter};

pub mod parser;

pub struct Charset {
  charset: Box<str>,
}

impl Charset {
  pub fn new(dictionary_text: &str) -> Result<Charset, Error> {
    let charset = Self::try_from(dictionary_text)?;
    Ok(charset)
  }

  pub fn transform_str(&self, content: &str) -> String {
    self.transform_bytes(content.as_bytes())
  }

  pub fn transform_bytes(&self, content: &[u8]) -> String {
    let lookup_table: Vec<char> = self.charset.chars().collect();
    let modulus = lookup_table.len();
    let mapped = content.iter().map(|item| {
      let lookup_idx = (*item) as usize % modulus;

      // No need for bound checking since lookup table is local and immutable
      unsafe {
        let value = lookup_table.get_unchecked(lookup_idx);
        *value
      }
    });

    String::from_iter(mapped)
  }
}

impl TryFrom<&str> for Charset {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let characters = parse(value)?;
    Ok(Charset {
      charset: characters.into_boxed_str(),
    })
  }
}

impl Display for Charset {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.charset.as_ref())
  }
}
