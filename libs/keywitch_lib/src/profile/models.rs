use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct TagItem {
  pub key_id: i64,
  pub name: String,
}
