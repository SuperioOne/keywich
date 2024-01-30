use crate::errors::Error;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::sqlx_macros::migrate;
use std::path::PathBuf;
use std::str::FromStr;
use crate::profile::charsets::Charsets;

pub mod charsets;
pub mod keys;
pub mod utils;

pub struct ProfileDB {
  pool: SqlitePool,
  migrator: Migrator,
}

impl ProfileDB {
  async fn connect(db_path: PathBuf) -> Result<Self, Error> {
    if db_path.is_dir() {
      return Err(Error::InvalidDatabasePath(db_path));
    }

    let path = db_path.to_str();

    match path {
      None => Err(Error::InvalidDatabasePath(db_path)),
      Some(path) => {
        let mut options = SqliteConnectOptions::from_str(path)?;
        options = options
          .pragma("key", "TODO:_get_rid_of_test_password")
          .foreign_keys(true)
          .read_only(false);

        let pool = SqlitePool::connect_with(options).await?;
        let migrator = migrate!("src/migrations");

        Ok(ProfileDB { pool, migrator })
      }
    }
  }

  async fn charsets(&self) -> Result<Charsets, Error> {
    let connection = self.pool.acquire().await?;

    Ok(Charsets{
      connection: connection
    })
  }
}
