use crate::profile::charsets::CharsetItem;
use crate::profile::keys::KeyItem;
use std::io::Write;
use std::path::PathBuf;

pub(crate) trait BackupWriter: Write {
  type WriterError;

  fn write_charsets(&mut self, charsets: &[CharsetItem]) -> Result<(), Self::WriterError>;
  fn write_keys(&mut self, keys: &[KeyItem]) -> Result<(), Self::WriterError>;
  fn write_icons(&mut self, paths: &[(String, PathBuf)]) -> Result<(), Self::WriterError>;
  fn finish(self) -> Result<(), Self::WriterError>;
}
