use std::collections::HashSet;

pub struct Charset
{
  pub charset: String,
  pub modulus: usize,
}

impl Charset
{
  pub(crate) fn transform_str(&self, content: &str) -> String
  {
    self.transform_bytes(content.as_bytes())
  }

  pub(crate) fn transform_bytes(&self, content: &[u8]) -> String
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

enum SymbolType
{
  Numeric(char),
  AlphaUppercase(char),
  AlphaLowercase(char),
}

macro_rules! symbol_type {
    ($arg:expr) => {
      match $arg
      {
        Some(lowercase @ 'a'..='z') => SymbolType::AlphaLowercase(lowercase),
        Some(uppercase @ 'A'..='Z') => SymbolType::AlphaUppercase(uppercase),
        Some(numeric @ '0'..='9') => SymbolType::Numeric(numeric),
        _ => panic!()
      }
    };
}

enum Token
{
  Range { start: char, end: char },
  Char(char),
}

fn parse(dictionary: &str)
{
  let mut iter = dictionary.chars().peekable();
  let mut set: HashSet<char> = HashSet::new();

  while let Some(token) = iter.next()
  {
    // Is escape
    if token == '\\'
    {
      // get next
      if let Some(char) = iter.next()
      {
        match char
        {
          '\\' => set.insert('\\'),
          '-' => set.insert('-'),
          _ => { true }
        };
      } else {
        // Error unsupported escape sequence
        todo!()
      }
    }
    // peek next to check if it's a range
    else if let Some(&'-') = iter.peek()
    {
      let range_start = symbol_type!(Some(token));
      _ = iter.next();
      let range_end = symbol_type!(iter.next());

      // Do the calculation here
      match (range_start, range_end)
      {
        (SymbolType::AlphaLowercase(start), SymbolType::AlphaLowercase(end)) => Token::Range { start, end },
        (SymbolType::AlphaUppercase(start), SymbolType::AlphaUppercase(end)) => Token::Range { start, end },
        (SymbolType::Numeric(start), SymbolType::Numeric(end)) => Token::Range { start, end },
        _ => Token::Range { start: ' ', end: ' ' }
      };
    }
    // it's a singular symbol, simply push
    else {
      set.insert(token);
    }
  }
}

