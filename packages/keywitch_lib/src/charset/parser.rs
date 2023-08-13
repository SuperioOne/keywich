use std::collections::{BTreeSet, HashSet};
use crate::errors::KeywitchError;

// "A..Za..z0..9-=#./,£$%~[](){} "
trait SymbolCollection
{
  fn insert_symbol(&mut self, symbol: SymbolType) -> bool;
}

impl SymbolCollection for BTreeSet<char>
{
  fn insert_symbol(&mut self, symbol: SymbolType) -> bool {
    match symbol {
      SymbolType::Numeric(char) => self.insert(char),
      SymbolType::AlphaUppercase(char) => self.insert(char),
      SymbolType::AlphaLowercase(char) => self.insert(char),
      SymbolType::Symbol(char) => self.insert(char),
      SymbolType::Range { start, end } => {
        for ch in start..=end
        {
          self.insert(ch);
        }

        true
      }
    }
  }
}

enum SymbolType
{
  Numeric(char),
  AlphaUppercase(char),
  AlphaLowercase(char),
  Symbol(char),
  Range { start: char, end: char },
}

pub fn parse(dictionary_text: &str) -> Result<String, KeywitchError>
{
  let mut iter = dictionary_text.chars().peekable();
  let mut set: BTreeSet<char> = BTreeSet::new();

  while let Some(input_char) = iter.next()
  {
    let current_symbol = parse_symbol(input_char);

    if let Some('.') = iter.peek()
    {
      iter.next();
      if let Some('.') = iter.peek()
      {
        iter.next(); //skipping second '.'
        if let Some(char) = iter.next()
        {
          let range_token: SymbolType = parse_range((input_char, char))?;
          set.insert_symbol(range_token);
        } else {
          return Err(KeywitchError::ParserInvalidRange);
        }
      } else {
        set.insert_symbol(current_symbol);
        set.insert_symbol(SymbolType::Symbol('.'));
      }
    } else {
      set.insert_symbol(current_symbol);
    }
  }

  Ok(String::from_iter(set.into_iter()))
}

fn parse_range(range: (char, char)) -> Result<SymbolType, KeywitchError>
{
  match range
  {
    (range_start @ 'a'..='z', range_end @ 'a'..='z') =>
      Ok(SymbolType::Range {
        start: range_start,
        end: range_end,
      }),
    (range_start @ 'A'..='Z', range_end @ 'A'..='Z') =>
      Ok(SymbolType::Range {
        start: range_start,
        end: range_end,
      }),
    (range_start @ '0'..='9', range_end @ '0'..='9') =>
      Ok(SymbolType::Range {
        start: range_start,
        end: range_end,
      }),
    _ => Err(KeywitchError::ParserInvalidRange)
  }
}

fn parse_symbol(input: char) -> SymbolType
{
  match input
  {
    lowercase @ 'a'..='z' => SymbolType::AlphaLowercase(lowercase),
    uppercase @ 'A'..='Z' => SymbolType::AlphaUppercase(uppercase),
    numeric @ '0'..='9' => SymbolType::Numeric(numeric),
    symbol => SymbolType::Symbol(symbol),
  }
}