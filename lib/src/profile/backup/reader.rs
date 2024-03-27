use std::io::Write;

use super::BackupManifest;
use crate::profile::charsets::CharsetItem;
use crate::profile::keys::KeyItem;

#[derive(Debug)]
pub struct IconDetails {
  pub name: String,
  pub full_name: String,
}

pub trait BackupReader {
  type ReaderError;

  fn verify_digest(&mut self, key: &[u8]) -> Result<bool, Self::ReaderError>;
  fn manifest(&mut self) -> Result<BackupManifest, Self::ReaderError>;
  fn charsets(&mut self) -> Result<Vec<CharsetItem>, Self::ReaderError>;
  fn keys(&mut self) -> Result<Vec<KeyItem>, Self::ReaderError>;
  fn icons(&mut self) -> Result<Vec<IconDetails>, Self::ReaderError>;
  fn copy_section_to<T>(&mut self, name: &str, target: T) -> Result<usize, Self::ReaderError>
  where
    T: Write;
}
