use crate::errors::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::sqlx_macros::migrate;
use std::str::FromStr;
use std::time::Duration;

pub mod charsets;
pub mod keys;
pub mod utils;

// TODO: Create a trait and implement other databases when next development phase begins for remote api.
pub struct ProfileDB {
  pool: SqlitePool,
}

pub struct ProfileDBSqliteOptions {
  pub password: Option<String>,
  pub busy_timeout: Option<Duration>,
  pub disable_migrate: bool,
}

impl Default for ProfileDBSqliteOptions {
  fn default() -> Self {
    Self {
      disable_migrate: false,
      busy_timeout: Some(Duration::from_secs(30)),
      password: None,
    }
  }
}

impl ProfileDB {
  pub async fn connect(connection_str: &str) -> Result<Self, Error> {
    let db = Self::connect_with(connection_str, ProfileDBSqliteOptions::default()).await?;
    Ok(db)
  }

  pub async fn connect_with(
    connection_str: &str,
    sqlite_options: ProfileDBSqliteOptions,
  ) -> Result<Self, Error> {
    let ProfileDBSqliteOptions {
      busy_timeout,
      password,
      disable_migrate,
    } = sqlite_options;

    let mut options = SqliteConnectOptions::from_str(connection_str)?;
    options = options
      .create_if_missing(true)
      .foreign_keys(true)
      .busy_timeout(busy_timeout.unwrap_or(Duration::from_secs(30)))
      .read_only(false);

    if let Some(pass) = password {
      options = options.pragma("key", pass);
    }

    let pool = SqlitePool::connect_with(options).await?;

    if !disable_migrate {
      let migrator = migrate!("src/migrations");
      migrator.run(&pool).await?;
    }

    Ok(ProfileDB { pool })
  }
}
