use crate::errors::Error;
use serde::Serialize;
use sqlx::{query, query_as, FromRow, SqliteConnection};

#[derive(Debug, FromRow, Serialize)]
pub struct CharsetItem {
  pub name: String,
  pub charset: String,
  pub description: Option<String>,
}

pub struct Charsets {
  connection: SqliteConnection,
}

impl Charsets {
  pub fn new(connection: SqliteConnection) -> Self {
    Self { connection }
  }

  pub async fn get_charsets(&mut self) -> Result<Vec<CharsetItem>, Error> {
    let result = query_as!(CharsetItem, "SELECT name,charset,description FROM charsets",)
      .fetch_all(&mut self.connection)
      .await
      .map_err(|err| err.into())?;

    Ok(result)
  }

  pub async fn insert_charset(&mut self, item: CharsetItem) -> Result<u64, Error> {
    _ = crate::charset::parser::parse(&item.charset)?;

    let result = query!(
      "INSERT INTO charsets (name,charset,description) VALUES (?,?,?)",
      item.name,
      item.charset,
      item.description
    )
    .execute(&mut self.connection)
    .await
    .map_err(|err| err.into())?;

    Ok(result.rows_affected())
  }

  pub async fn delete_charset(&mut self, name: &str) -> Result<u64, Error> {
    let result = query!("DELETE FROM charsets WHERE name = ?", name)
      .execute(&mut self.connection)
      .await
      .map_err(|err| err.into())?;

    Ok(result.rows_affected())
  }
}

#[cfg(test)]
mod test {
  use crate::profile::charsets::{CharsetItem, Charsets};
  use sqlx::Connection;
  use sqlx::SqliteConnection;

  #[tokio::test]
  async fn read_test() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut charset_api = Charsets::new(sqlite_connection);
    let charset_items = charset_api.get_charsets().await.unwrap();

    assert!(!charset_items.is_empty())
  }

  #[tokio::test]
  async fn insert_and_test() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut charset_api = Charsets::new(sqlite_connection);
    let inserted = charset_api
      .insert_charset(CharsetItem {
        charset: "a..zA..Z".into(),
        description: Some("Description".into()),
        name: "__test_insert".into(),
      })
      .await
      .unwrap();
    let deleted_row = charset_api.delete_charset("__test_insert").await.unwrap();

    assert_eq!(1, inserted);
    assert_eq!(1, deleted_row);
  }
}
