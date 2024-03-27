use super::{
  reader::{BackupReader, IconDetails},
  writer::BackupWriter,
  BackupManifest,
};
use crate::{
  errors::Error,
  profile::{charsets::CharsetItem, keys::KeyItem},
};
use hmac::Mac;
use log::{debug, error};
use std::{
  io::{ErrorKind, Read, Seek, Write},
  path::PathBuf,
};
use zip::{write::FileOptions, ZipArchive, ZipWriter};

type Hmac256 = hmac::Hmac<sha2::Sha256>;

const SECTION_KEYS: &str = "keys";
const SECTION_CONTENTS: &str = "contents";
const SECTION_CHARSETS: &str = "charsets";
const SECTION_MANIFEST: &str = "manifest";

pub struct FileBackupWriter<T>
where
  T: Write + Seek,
{
  inner: ZipWriter<T>,
  sign_gen: Hmac256,
  manifest: BackupManifest,
}

impl<T> Write for FileBackupWriter<T>
where
  T: Write + Seek,
{
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.inner.write(buf)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.inner.flush()
  }
}

impl<T> FileBackupWriter<T>
where
  T: Write + Seek,
{
  pub fn new(writer: T, key: &[u8]) -> Result<Self, Error> {
    let zip_writer = ZipWriter::new(writer);
    let mac = Hmac256::new_from_slice(key).map_err(|err| {
      error!("{}", err);
      Error::BackupError(err.to_string())
    })?;

    Ok(Self {
      inner: zip_writer,
      manifest: BackupManifest::default(),
      sign_gen: mac,
    })
  }
}

impl<T> BackupWriter for FileBackupWriter<T>
where
  T: Write + Seek,
{
  type WriterError = Error;

  fn write_charsets(&mut self, charsets: &[CharsetItem]) -> Result<(), Self::WriterError> {
    let zip_options = FileOptions::default()
      .compression_method(zip::CompressionMethod::DEFLATE)
      .unix_permissions(0o644);

    let bytes = serde_json::to_vec(charsets)?;

    self.sign_gen.update(&bytes);
    self.manifest.files.push(SECTION_CHARSETS.to_string());
    self.inner.start_file(SECTION_CHARSETS, zip_options)?;
    self.inner.write_all(&bytes).map_err(|err| {
      error!("{}", err);
      Error::BackupError(err.to_string())
    })?;

    debug!("Charsets written to backup file.");

    Ok(())
  }

  fn write_keys(&mut self, keys: &[KeyItem]) -> Result<(), Self::WriterError> {
    let zip_options = FileOptions::default()
      .compression_method(zip::CompressionMethod::DEFLATE)
      .unix_permissions(0o644);

    let bytes = serde_json::to_vec(keys)?;

    self.sign_gen.update(&bytes);
    self.manifest.files.push(SECTION_KEYS.to_string());
    self.inner.start_file(SECTION_KEYS, zip_options)?;
    self.inner.write_all(&bytes).map_err(|err| {
      error!("{}", err);
      Error::BackupError(err.to_string())
    })?;

    debug!("Keys written to backup file.");

    Ok(())
  }

  fn write_icons(&mut self, paths: &[(String, PathBuf)]) -> Result<(), Self::WriterError> {
    let zip_options = FileOptions::default()
      .compression_method(zip::CompressionMethod::DEFLATE)
      .unix_permissions(0o644);

    self.inner.add_directory(SECTION_CONTENTS, zip_options)?;

    for (name, path) in paths {
      let mut fd = std::fs::File::open(path).map_err(|err| {
        error!("Unable to open icon file, {}", err);
        Error::BackupError(err.to_string())
      })?;

      let mut buffer = [0u8; 4096];
      let file_name = format!("{}/{}", SECTION_CONTENTS, name);

      self.inner.start_file(&file_name, zip_options)?;

      'reader: loop {
        match fd.read(&mut buffer) {
          Ok(0) => {
            debug!("Icon file {:?} written to backup file.", name);

            self.manifest.files.push(file_name);
            break 'reader;
          }
          Ok(size) => {
            let buf = &buffer[..size];

            'writer: loop {
              match self.inner.write_all(buf) {
                Err(err) if err.kind() == ErrorKind::Interrupted => continue 'writer,
                Err(err) => {
                  error!("{}", err);
                  return Err(Error::BackupError(err.to_string()));
                }
                Ok(()) => {
                  self.sign_gen.update(buf);
                  break 'writer;
                }
              };
            }
          }
          Err(err) if err.kind() == ErrorKind::WouldBlock => continue 'reader,
          Err(err) if err.kind() == ErrorKind::Interrupted => continue 'reader,
          Err(err) => {
            error!("Unable to read icon file content, {}", err);
            return Err(Error::BackupError(err.to_string()));
          }
        }
      }
    }

    Ok(())
  }

  fn finish(mut self) -> Result<(), Self::WriterError> {
    let zip_options = FileOptions::default()
      .compression_method(zip::CompressionMethod::DEFLATE)
      .unix_permissions(0o644);

    let digest = self.sign_gen.finalize().into_bytes();
    let digest_hex = format!("{:x}", digest);

    debug!("file digest value is {}", &digest_hex);

    self.manifest.digest = Some(digest_hex);
    self.inner.start_file(SECTION_MANIFEST, zip_options)?;
    serde_json::to_writer(&mut self.inner, &self.manifest)?;
    self.inner.finish()?;

    Ok(())
  }
}

pub struct FileBackupReader<T>
where
  T: Read + Seek,
{
  inner: ZipArchive<T>,
  manifest: BackupManifest,
}

impl<T> FileBackupReader<T>
where
  T: Read + Seek,
{
  pub fn new(reader: T) -> Result<Self, Error> {
    let mut zip_archive = ZipArchive::new(reader)?;
    let manifest_file = zip_archive.by_name(SECTION_MANIFEST)?;
    let manifest: BackupManifest = serde_json::from_reader(manifest_file)?;

    Ok(Self {
      inner: zip_archive,
      manifest,
    })
  }
}

impl<T> BackupReader for FileBackupReader<T>
where
  T: Seek + Read,
{
  type ReaderError = Error;

  fn manifest(&mut self) -> Result<super::BackupManifest, Self::ReaderError> {
    Ok(self.manifest.clone())
  }

  fn charsets(&mut self) -> Result<Vec<crate::profile::charsets::CharsetItem>, Self::ReaderError> {
    match self.inner.by_name(SECTION_CHARSETS) {
      Ok(file) => {
        let charsets: Vec<CharsetItem> = serde_json::from_reader(file)?;
        Ok(charsets)
      }
      Err(zip::result::ZipError::FileNotFound) => Ok(Vec::new()),
      Err(err) => {
        error!("Reading the charset backup failed, {}", err);
        Err(err.into())
      }
    }
  }

  fn keys(&mut self) -> Result<Vec<crate::profile::keys::KeyItem>, Self::ReaderError> {
    match self.inner.by_name(SECTION_KEYS) {
      Ok(file) => {
        let keys: Vec<KeyItem> = serde_json::from_reader(file)?;
        Ok(keys)
      }
      Err(zip::result::ZipError::FileNotFound) => Ok(Vec::new()),
      Err(err) => {
        error!("Reading the charset backup failed, {}", err);
        Err(err.into())
      }
    }
  }

  fn icons(&mut self) -> Result<Vec<IconDetails>, Self::ReaderError> {
    let prefix = format!("{}/", SECTION_CONTENTS);
    let icon_files: Vec<IconDetails> = self
      .manifest
      .files
      .iter()
      .flat_map(|e| {
        e.strip_prefix(&prefix).map(|v| IconDetails {
          name: v.to_owned(),
          full_name: e.to_owned(),
        })
      })
      .collect();

    Ok(icon_files)
  }

  fn copy_section_to<W>(&mut self, name: &str, mut target: W) -> Result<usize, Self::ReaderError>
  where
    W: Write,
  {
    let mut file = self.inner.by_name(name)?;
    let mut buf = [0u8; 4096];
    let mut written: usize = 0;

    'reader: loop {
      match file.read(&mut buf) {
        Ok(0) => {
          target.flush().map_err(|err| {
            error!("{}", err);
            Error::BackupError(err.to_string())
          })?;

          break 'reader;
        }
        Ok(size) => 'writer: loop {
          match target.write(&buf[..size]) {
            Ok(0) => {
              return Err(Error::BackupError(String::from(
                "Cannot write to target stream. Stream might be closed or not accessible.",
              )));
            }
            Ok(written_size) => {
              written += written_size;
              break 'writer;
            }
            Err(err) if err.kind() == ErrorKind::Interrupted => continue 'writer,
            Err(err) if err.kind() == ErrorKind::WouldBlock => continue 'writer,
            Err(err) => {
              error!("{}", err);
              return Err(Error::BackupError(format!(
                "unable to write backup content to target writer, {}",
                err
              )));
            }
          };
        },
        Err(err) if err.kind() == ErrorKind::WouldBlock => continue 'reader,
        Err(err) if err.kind() == ErrorKind::Interrupted => continue 'reader,
        Err(err) => {
          error!("{}", err);
          return Err(Error::BackupError(format!(
            "Unable to read backup content, {}",
            err
          )));
        }
      };
    }

    Ok(written)
  }

  fn verify_digest(&mut self, key: &[u8]) -> Result<bool, Self::ReaderError> {
    match &self.manifest.digest {
      Some(manifest_digest) => {
        let mut mac = Hmac256::new_from_slice(key).map_err(|err| {
          error!("{}", err);
          Error::BackupError(err.to_string())
        })?;

        for file_name in &self.manifest.files {
          let mut fd = self.inner.by_name(file_name)?;
          let mut buf: Vec<u8> = Vec::new();

          fd.read_to_end(&mut buf).map_err(|err| {
            error!("{}", &err);
            Error::BackupError(err.to_string())
          })?;

          mac.update(&buf);
        }

        let digest = mac.finalize().into_bytes();
        let digest_hex = format!("{:x}", digest);

        Ok(manifest_digest.eq_ignore_ascii_case(&digest_hex))
      }
      None => Ok(false),
    }
  }
}
