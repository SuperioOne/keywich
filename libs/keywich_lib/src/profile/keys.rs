use crate::charset::validate_charset;
use crate::errors::Error;
use crate::profile::utils::tag_list::TagList;
use crate::profile::utils::timestamp::get_unix_timestamp;
use crate::profile::ProfileDB;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Connection, FromRow, QueryBuilder, Sqlite, SqliteConnection};
use validator::Validate;

struct SearchIndex {
  domain: String,
  id: i64,
  notes: Option<String>,
  tags: String,
  username: String,
}

enum SearchIndexOp {
  DeleteIndex(i64),
  UpdateIndex(SearchIndex),
  CreateIndex(SearchIndex),
}

#[derive(Debug)]
pub struct SearchQuery {
  pub username_tokens: Vec<Box<str>>,
  pub domain_tokens: Vec<Box<str>>,
  pub tag_tokens: Vec<Box<str>>,
  pub fts_tokens: Vec<Box<str>>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct KeyItem {
  pub id: i64,
  pub pinned: bool,
  pub target_size: i64,
  pub revision: i64,
  pub charset: String,
  pub domain: String,
  pub username: String,
  pub notes: Option<String>,
  pub created_at: i64,
  pub custom_icon: Option<String>,
  pub version: String,

  // Query default value must be an empty array for tags since current sqlx version does not support nullable json columns.
  // See https://github.com/launchbadge/sqlx/issues/2849
  #[sqlx(json)]
  pub tags: TagList,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct KeyData {
  #[validate(range(min = 1, max = 64))]
  pub target_size: i64,
  pub revision: i64,
  #[validate(length(min = 1), custom = "validate_charset")]
  pub charset: String,
  #[validate(length(min = 1))]
  pub domain: String,
  #[validate(length(min = 1))]
  pub username: String,
  pub notes: Option<String>,
  pub custom_icon: Option<String>,
  #[validate(length(min = 1))]
  pub version: String,
  pub tags: TagList,
}

impl ProfileDB {
  pub async fn get_key_by_id(&self, key_id: i64) -> Result<Option<KeyItem>, Error> {
    let mut conn = self.pool.acquire().await?;
    let row = query_as::<Sqlite, KeyItem>(
      "SELECT
        keys.id,
        keys.pinned,
        keys.target_size,
        keys.revision,
        keys.charset,
        keys.domain,
        keys.username,
        keys.notes,
        keys.created_at,
        keys.custom_icon,
        keys.version,
        ifnull(vw_tag_list.tags, json_array()) as tags
      FROM keys
      LEFT JOIN vw_tag_list ON vw_tag_list.key_id = keys.id
      WHERE keys.id = ? LIMIT 1;",
    )
    .bind(key_id)
    .fetch_one(&mut *conn)
    .await;

    match row {
      Ok(result) => Ok(Some(result)),
      Err(sqlx::Error::RowNotFound) => Ok(None),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn get_keys(&self, pinned_only: bool) -> Result<Vec<KeyItem>, Error> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
      "SELECT
        keys.id,
        keys.pinned,
        keys.target_size,
        keys.revision,
        keys.charset,
        keys.domain,
        keys.username,
        keys.notes,
        keys.created_at,
        keys.custom_icon,
        keys.version,
        ifnull(vw_tag_list.tags, json_array()) as tags
      FROM keys
      LEFT JOIN vw_tag_list ON vw_tag_list.key_id = keys.id",
    );

    if pinned_only {
      query_builder
        .push(" WHERE keys.pinned = ")
        .push_bind(pinned_only);
    }

    let query = query_builder.build_query_as::<KeyItem>();
    let mut conn = self.pool.acquire().await?;
    let result: Vec<KeyItem> = query.fetch_all(&mut *conn).await?;

    Ok(result)
  }

  pub async fn search_keys(&self, search_query: SearchQuery) -> Result<Vec<KeyItem>, Error> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
      "SELECT
        keys.id,
        keys.pinned,
        keys.target_size,
        keys.revision,
        keys.charset,
        keys.domain,
        keys.username,
        keys.notes,
        keys.created_at,
        keys.custom_icon,
        keys.version,
        ifnull(vw_tag_list.tags, json_array()) as tags
      FROM keys
      LEFT JOIN vw_tag_list on keys.id = vw_tag_list.key_id ",
    );

    if let Some(query) = search_query.to_fts_query() {
      query_builder.push(
        "JOIN (SELECT DISTINCT ROWID as s_key, rank
           FROM search_index
           WHERE search_index MATCH ",
      );
      query_builder.push_bind(query);
      query_builder.push(" ) ON s_key = keys.id ORDER BY rank;");
    }

    let mut conn = self.pool.acquire().await?;
    let query = query_builder.build_query_as::<KeyItem>();
    let result: Vec<KeyItem> = query.fetch_all(&mut *conn).await?;

    Ok(result)
  }

  pub async fn delete_key(&self, key_id: i64) -> Result<bool, Error> {
    let mut conn = self.pool.acquire().await?;
    let mut transaction = conn.begin().await?;

    query!("DELETE FROM tags WHERE tags.key_id = ?", key_id)
      .execute(&mut *transaction)
      .await?;

    let key_result = query!("DELETE FROM keys WHERE keys.id = ?", key_id)
      .execute(&mut *transaction)
      .await?;

    Self::sync_search_index(&mut *transaction, SearchIndexOp::DeleteIndex(key_id)).await?;

    transaction.commit().await?;

    Ok(key_result.rows_affected() > 0)
  }

  pub async fn insert_key(&self, item: KeyData) -> Result<i64, Error> {
    item.validate()?;

    let now: i64 = get_unix_timestamp()?;
    let mut conn = self.pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    let key_insert = query!(
      "INSERT INTO keys
        (pinned, target_size, revision, charset, domain, username, notes, created_at, custom_icon, version) VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      false,
      item.target_size,
      item.revision,
      item.charset,
      item.domain,
      item.username,
      item.notes,
      now,
      item.custom_icon,
      item.version
    )
      .execute(&mut *transaction)
      .await?;

    let key_id = key_insert.last_insert_rowid();

    if !item.tags.is_empty() {
      let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO tags (name, key_id) ");

      query_builder.push_values(item.tags.iter(), |mut b, tag| {
        b.push_bind(tag.as_ref());
        b.push_bind(key_id);
      });

      let query = query_builder.build();
      query.execute(&mut *transaction).await?;
    }

    Self::sync_search_index(
      &mut *transaction,
      SearchIndexOp::CreateIndex(SearchIndex {
        id: key_id,
        notes: item.notes,
        username: item.username,
        domain: item.domain,
        tags: item.tags.join(' '),
      }),
    )
    .await?;

    transaction.commit().await?;

    Ok(key_id)
  }

  pub async fn update_key(&self, key_id: i64, item: KeyData) -> Result<(), Error> {
    item.validate()?;

    let mut conn = self.pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    query!(
      "UPDATE keys SET
        (target_size, revision, charset, domain, username, notes, custom_icon, version) =
        (?, ?, ?, ?, ?, ?, ?, ?)
      WHERE keys.id = ?;",
      item.target_size,
      item.revision,
      item.charset,
      item.domain,
      item.username,
      item.notes,
      item.custom_icon,
      item.version,
      key_id
    )
    .execute(&mut *transaction)
    .await?;

    let existing_tags = query!("SELECT name FROM tags WHERE key_id = ?", key_id)
      .fetch_all(&mut *transaction)
      .await?;

    let existing_tags = TagList::from(existing_tags.into_iter().flat_map(|e| e.name));
    let new_tags = item.tags.difference(&existing_tags);
    let orphan_tags = existing_tags.difference(&item.tags);

    if orphan_tags.len() > 0 {
      let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("DELETE FROM tags WHERE (key_id, name) IN ");

      query_builder.push_tuples(orphan_tags.iter(), |mut b, tag| {
        b.push_bind(key_id);
        b.push_bind(tag.as_ref());
      });

      let query = query_builder.build();
      query.execute(&mut *transaction).await?;
    }

    if new_tags.len() > 0 {
      let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO tags (key_id, name) ");

      query_builder.push_values(new_tags.iter(), |mut b, tag| {
        b.push_bind(key_id);
        b.push_bind(tag.as_ref());
      });

      let query = query_builder.build();
      query.execute(&mut *transaction).await?;
    }

    Self::sync_search_index(
      &mut *transaction,
      SearchIndexOp::UpdateIndex(SearchIndex {
        id: key_id,
        notes: item.notes,
        username: item.username,
        domain: item.domain,
        tags: item.tags.join(' '),
      }),
    )
    .await?;
    transaction.commit().await?;

    Ok(())
  }

  pub async fn update_pin_status(&self, key_id: i64, pin_status: bool) -> Result<(), Error> {
    let mut conn = self.pool.acquire().await?;
    query!(
      "UPDATE keys SET pinned = ? WHERE keys.id = ?;",
      pin_status,
      key_id
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
  }

  async fn sync_search_index(conn: &mut SqliteConnection, op: SearchIndexOp) -> Result<(), Error> {
    match op {
      SearchIndexOp::DeleteIndex(id) => {
        query!("DELETE FROM search_index WHERE ROWID = ?", id)
          .execute(conn)
          .await?;
      }
      SearchIndexOp::UpdateIndex(update_values) => {
        query!(
          "UPDATE search_index SET (domain, username, tags, notes) = (?,?,?,?) WHERE ROWID = ?",
          update_values.domain,
          update_values.username,
          update_values.tags,
          update_values.notes,
          update_values.id
        )
        .execute(conn)
        .await?;
      }
      SearchIndexOp::CreateIndex(create_values) => {
        query!(
          "INSERT INTO search_index (ROWID, domain, username, tags, notes) VALUES (?,?,?,?,?);",
          create_values.id,
          create_values.domain,
          create_values.username,
          create_values.tags,
          create_values.notes,
        )
        .execute(conn)
        .await?;
      }
    }

    Ok(())
  }
}

impl SearchQuery {
  pub fn new(query: &str) -> Self {
    let mut username_tokens: Vec<Box<str>> = Vec::new();
    let mut domain_tokens: Vec<Box<str>> = Vec::new();
    let mut tag_tokens: Vec<Box<str>> = Vec::new();
    let mut fts_tokens: Vec<Box<str>> = Vec::new();

    for token in query.split_whitespace() {
      if let Some(tag_token) = token.strip_prefix("tag:") {
        tag_tokens.push(Box::from(tag_token));
      } else if let Some(domain_token) = token.strip_prefix("domain:") {
        domain_tokens.push(Box::from(domain_token));
      } else if let Some(usr_token) = token.strip_prefix("username:") {
        username_tokens.push(Box::from(usr_token));
      } else {
        fts_tokens.push(Box::from(token));
      }
    }

    Self {
      fts_tokens,
      username_tokens,
      domain_tokens,
      tag_tokens,
    }
  }

  pub fn to_fts_query(&self) -> Option<String> {
    let mut queries: Vec<String> = Vec::new();

    if !self.fts_tokens.is_empty() {
      queries.push(format!(
        "({{domain username tags notes}}: {})",
        Self::join_tokens(&self.fts_tokens, " OR ")
      ));
    }

    if !self.username_tokens.is_empty() {
      queries.push(format!(
        "({{username}}: {})",
        Self::join_tokens(&self.username_tokens, " OR ")
      ));
    }

    if !self.domain_tokens.is_empty() {
      queries.push(format!(
        "({{domain}}: {})",
        Self::join_tokens(&self.domain_tokens, " OR ")
      ));
    }

    if !self.tag_tokens.is_empty() {
      queries.push(format!(
        "({{tags}}: {})",
        Self::join_tokens(&self.tag_tokens, " OR ")
      ));
    }

    if queries.is_empty() {
      None
    } else {
      Some(queries.join(" AND "))
    }
  }

  fn join_tokens<T>(tokens: &[T], separator: &str) -> String
  where
    T: AsRef<str>,
  {
    let mut result = String::new();

    for (index, token) in tokens.iter().enumerate() {
      let normalized = format!("\"{}\"", token.as_ref().replace("\"", "\"\""));
      result.push_str(&normalized);

      if index != tokens.len() - 1 {
        result.push_str(separator);
      }
    }

    result
  }
}

impl<T> From<T> for SearchQuery
where
  T: AsRef<str>,
{
  fn from(value: T) -> Self {
    Self::new(value.as_ref())
  }
}
