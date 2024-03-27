use self::{
  file_backup::{FileBackupReader, FileBackupWriter},
  reader::BackupReader,
  writer::BackupWriter,
};
use super::{charsets::CharsetItem, keys::KeyItem, ProfileDB};
use crate::errors::Error;
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{query, Acquire, QueryBuilder, Sqlite};
use std::{
  fs::OpenOptions,
  io::{BufReader, BufWriter},
  path::PathBuf,
};

pub mod file_backup;
pub mod reader;
pub mod writer;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackupManifest {
  pub version: u8,
  pub files: Vec<String>,
  pub digest: Option<String>,
}

impl BackupManifest {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Default for BackupManifest {
  fn default() -> Self {
    Self {
      version: 1,
      files: Vec::new(),
      digest: None,
    }
  }
}

pub enum BackupTarget {
  File(PathBuf),
  // Http Soon™
}

pub struct BackupOptions {
  pub content_dir: PathBuf,
  pub target: BackupTarget,
  pub sign_key: Box<[u8]>,
}

pub struct RestoreOptions {
  pub content_dir: PathBuf,
  pub target: BackupTarget,
}

impl ProfileDB {
  pub async fn backup(&self, options: BackupOptions) -> Result<(), Error> {
    let BackupOptions {
      content_dir,
      target,
      sign_key,
    } = options;

    let keys = self.get_keys(false).await?;
    let charsets = self.get_charsets().await?;
    let icons: Vec<(String, PathBuf)> = keys
      .iter()
      .flat_map(|x| match &x.custom_icon {
        Some(icon_name) if !icon_name.is_empty() => {
          Some((String::from(icon_name), content_dir.join(icon_name)))
        }
        _ => None,
      })
      .collect();

    match target {
      BackupTarget::File(path) => {
        let open_options = &mut std::fs::OpenOptions::new();
        let fs_opt = open_options.create(true).write(true);
        let fd = fs_opt.open(path).map_err(|err| {
          error!("Unable to create target file, {}", err);
          Error::BackupError(err.to_string())
        })?;

        let writer = BufWriter::new(fd);
        let mut backup_writer = FileBackupWriter::new(writer, &sign_key)?;

        backup_writer.write_keys(&keys)?;
        backup_writer.write_charsets(&charsets)?;
        backup_writer.write_icons(&icons)?;
        backup_writer.finish()?;
      }
    };

    Ok(())
  }

  pub async fn restore(&self, options: RestoreOptions) -> Result<(), Error> {
    let RestoreOptions {
      target,
      content_dir,
    } = options;

    if !content_dir.is_dir() {
      return Err(Error::BackupError(String::from(
        "Content target isn't directory.",
      )));
    }

    let mut backup_reader = match target {
      BackupTarget::File(path) => {
        let open_options = &mut std::fs::OpenOptions::new();
        let fs_opt = open_options.read(true);
        let fd = fs_opt.open(path).map_err(|err| {
          error!("Unable to create target file, {}", err);
          Error::BackupError(err.to_string())
        })?;

        let reader = BufReader::new(fd);
        FileBackupReader::new(reader)?
      }
    };

    let keys = backup_reader.keys()?;
    let charsets = backup_reader.charsets()?;
    let icons = backup_reader.icons()?;

    let mut conn = self.pool.acquire().await?;
    let mut transaction = conn.begin().await?;

    query!(
      "DELETE FROM tags;
       DELETE FROM keys;
       DELETE FROM charsets;
       DELETE FROM search_index;"
    )
    .execute(&mut *transaction)
    .await?;

    let mut key_query = create_key_query(&keys);
    key_query.build().execute(&mut *transaction).await?;

    let mut tag_query = create_tag_query(&keys);
    tag_query.build().execute(&mut *transaction).await?;

    let mut index_query = create_index_query(&keys);
    index_query.build().execute(&mut *transaction).await?;

    let mut charset_query = create_charset_query(&charsets);
    charset_query.build().execute(&mut *transaction).await?;

    transaction.commit().await?;

    for icon in icons {
      let fs_path = content_dir.join(&icon.name);
      let fd = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(fs_path)
        .map_err(|err| {
          error!("Icon {} {}", &icon.name, err);
          Error::BackupError(format!("Unable to open content file, {}", err))
        })?;

      backup_reader.copy_section_to(&icon.full_name, fd)?;
    }

    Ok(())
  }
}

fn create_key_query(keys: &[KeyItem]) -> QueryBuilder<'_, Sqlite> {
  let mut query_builder: QueryBuilder<Sqlite> =
    QueryBuilder::new("INSERT INTO keys (id, pinned, target_size, revision, charset, domain, username, notes, created_at, custom_icon, version) ");

  query_builder.push_values(keys.iter(), |mut b, key_item| {
    b.push_bind(key_item.id);
    b.push_bind(key_item.pinned);
    b.push_bind(key_item.target_size);
    b.push_bind(key_item.revision);
    b.push_bind(&key_item.charset);
    b.push_bind(&key_item.domain);
    b.push_bind(&key_item.username);
    b.push_bind(&key_item.notes);
    b.push_bind(key_item.created_at);
    b.push_bind(&key_item.custom_icon);
    b.push_bind(&key_item.version);
  });

  query_builder
}

fn create_tag_query(keys: &[KeyItem]) -> QueryBuilder<'_, Sqlite> {
  let mut query_builder: QueryBuilder<Sqlite> =
    QueryBuilder::new("INSERT INTO tags (name, key_id) ");
  let key_iter = keys.iter().flat_map(|e| e.tags.iter().map(|t| (t, e.id)));

  query_builder.push_values(key_iter, |mut b, (name, key_id)| {
    b.push_bind(name.as_ref());
    b.push_bind(key_id);
  });

  query_builder
}

fn create_index_query(keys: &[KeyItem]) -> QueryBuilder<'_, Sqlite> {
  let mut query_builder: QueryBuilder<Sqlite> =
    QueryBuilder::new("INSERT INTO search_index (ROWID, domain, username, tags, notes) ");

  query_builder.push_values(keys.iter(), |mut b, key_item| {
    b.push_bind(key_item.id);
    b.push_bind(&key_item.domain);
    b.push_bind(&key_item.username);
    b.push_bind(key_item.tags.join(' '));
    b.push_bind(&key_item.notes);
  });

  query_builder
}

fn create_charset_query(charsets: &[CharsetItem]) -> QueryBuilder<'_, Sqlite> {
  let mut query_builder: QueryBuilder<Sqlite> =
    QueryBuilder::new("INSERT INTO charsets (name, charset, description) ");

  query_builder.push_values(charsets.iter(), |mut b, charset_item| {
    b.push_bind(&charset_item.name);
    b.push_bind(&charset_item.charset);
    b.push_bind(&charset_item.description);
  });

  query_builder
}
