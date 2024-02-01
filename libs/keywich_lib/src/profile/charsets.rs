use crate::errors::Error;
use crate::profile::ProfileDB;
use serde::Serialize;
use sqlx::{query, query_as, FromRow};

#[derive(Debug, FromRow, Serialize)]
pub struct CharsetItem {
  pub name: String,
  pub charset: String,
  pub description: Option<String>,
}

impl ProfileDB {
  pub async fn get_charsets(&self) -> Result<Vec<CharsetItem>, Error> {
    let mut conn = self.pool.acquire().await?;
    let result = query_as!(CharsetItem, "SELECT name,charset,description FROM charsets",)
      .fetch_all(&mut *conn)
      .await?;

    Ok(result)
  }

  pub async fn insert_charset(&self, item: CharsetItem) -> Result<u64, Error> {
    _ = crate::charset::parser::parse(&item.charset)?;
    let mut conn = self.pool.acquire().await?;

    let result = query!(
      "INSERT INTO charsets (name,charset,description) VALUES (?,?,?)",
      item.name,
      item.charset,
      item.description
    )
    .execute(&mut *conn)
    .await?;

    Ok(result.rows_affected())
  }

  pub async fn delete_charset(&self, name: &str) -> Result<u64, Error> {
    let mut conn = self.pool.acquire().await?;
    let result = query!("DELETE FROM charsets WHERE name = ?", name)
      .execute(&mut *conn)
      .await?;

    Ok(result.rows_affected())
  }
}
