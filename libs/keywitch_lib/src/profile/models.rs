use sqlx::{FromRow};
use serde::{Serialize};

#[derive(Debug, FromRow, Serialize)]
pub struct PassMetadataItem
{
  pub pinned: bool,
  pub id: i64,
  pub target_size: u8,
  pub revision: u64,
  pub charset: String,
  pub domain: String,
  pub user_name: String,
  pub notes: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct CharsetItem
{
  pub id: i64,
  pub charset: String,
  pub description: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TagItem
{
  pub id: i64,
  pub tag_name: String,
}

