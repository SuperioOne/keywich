use crate::errors::Error;
use crate::profile::tag_list::TagList;
use serde::Serialize;
use sqlx::{query_as, FromRow, SqliteConnection};

#[derive(Debug, FromRow, Serialize)]
pub struct KeyItem {
  pub id: i64,
  pub pinned: Option<bool>,
  #[sqlx(try_from = "i64")]
  pub target_size: u64,
  #[sqlx(try_from = "i64")]
  pub revision: u64,
  pub charset: String,
  pub domain: String,
  pub user_name: String,
  pub notes: Option<String>,
  #[sqlx(try_from = "i64")]
  pub created_at: u64,
  pub custom_icon: Option<String>,
  #[sqlx(json)]
  pub tags: TagList,
}

pub struct Keys {
  connection: SqliteConnection,
  storage_path: Box<str>,
}

impl Keys {
  pub fn new(connection: SqliteConnection, storage_path: &str) -> Self {
    Self {
      connection,
      storage_path: Box::from(storage_path),
    }
  }

  pub async fn get_keys(&mut self) -> Result<Vec<KeyItem>, Error> {
    // Provide empty array for tags since current sqlx does not support nullable json https://github.com/launchbadge/sqlx/issues/2849
    let result: Vec<KeyItem> = query_as(
      "SELECT id,
      pinned,
      target_size,
      revision,
      charset,
      domain,
      user_name,
      notes,
      created_at,
      custom_icon,
      ifnull(tags, json_array()) as tags
  FROM keys
  LEFT JOIN vw_tag_list ON key_id = id WHERE id < 15",
    )
    .fetch_all(&mut self.connection)
    .await
    .map_err(|err| err.into())?;

    Ok(result)
  }
}

#[cfg(test)]
mod test {
  use crate::profile::keys::Keys;
  use sqlx::Connection;
  use sqlx::SqliteConnection;

  #[tokio::test]
  async fn read_test() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection, "/tmp");
    let keys = key_api.get_keys().await.unwrap();

    println!("{:?}", keys);
    assert!(true);
  }
}
