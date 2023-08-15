use crate::charset::parser::parse;
use crate::errors::KeywitchError;

pub mod parser;

pub struct Charset
{
  pub charset: String,
}

impl Charset
{
  pub fn new(dictionary_text: &str) -> Result<Charset, KeywitchError>
  {
    let characters = parse(dictionary_text)?;
    Ok(Charset { charset: characters })
  }

  pub fn transform_str(&self, content: &str) -> String
  {
    self.transform_bytes(content.as_bytes())
  }

  pub fn transform_bytes(&self, content: &[u8]) -> String
  {
    let lookup_table: Vec<char> = self.charset.chars().collect();
    let modulus = lookup_table.len();
    let mapped = content
      .iter()
      .map(|item| {
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
