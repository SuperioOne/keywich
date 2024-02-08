use crate::errors::Error;
use crate::profile::utils::tag_list::TagList;
use crate::profile::utils::timestamp::get_unix_timestamp;
use crate::profile::ProfileDB;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Connection, FromRow, QueryBuilder, Sqlite};

/// Inserts token if given flag is true and resets the flags afterward.
macro_rules! insert_token {
  ($query:expr, $flag:expr, $token:expr) => {
    let builder: &mut QueryBuilder<Sqlite> = $query;
    let token: &str = $token;
    let flag: &mut bool = $flag;

    if *flag {
      builder.push(token);
      *flag = false;
    }
  };
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

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyData {
  pub pinned: bool,
  pub target_size: i64,
  pub revision: i64,
  pub charset: String,
  pub domain: String,
  pub username: String,
  pub notes: Option<String>,
  pub custom_icon: Option<String>,
  pub version: String,
  pub tags: TagList,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
  pub username: Option<Vec<String>>,
  pub domain: Option<Vec<String>>,
  pub tag: Option<TagList>,
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
      "SELECT DISTINCT
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
      JOIN tags t on keys.id = t.key_id",
    );

    // TODO: When further requirements are clear, refactor query builder with a proc macro or extract it as a proper function.
    let mut prepend_token = false;
    let mut insert_where = true;

    if let Some(usernames) = search_query.username {
      insert_token!(&mut query_builder, &mut insert_where, " WHERE ");
      query_builder
        .push(" keys.username IN ")
        .push_tuples(usernames, |mut b, username| {
          b.push_bind(username);
        });
      prepend_token = true;
    }

    insert_token!(&mut query_builder, &mut prepend_token, " AND ");

    if let Some(domains) = search_query.domain {
      insert_token!(&mut query_builder, &mut insert_where, " WHERE ");
      query_builder
        .push(" keys.domain IN ")
        .push_tuples(domains, |mut b, domain| {
          b.push_bind(domain);
        });
      prepend_token = true;
    }

    insert_token!(&mut query_builder, &mut prepend_token, " AND ");

    if let Some(tags) = search_query.tag {
      insert_token!(&mut query_builder, &mut insert_where, " WHERE ");
      query_builder
        .push(" t.name IN ")
        .push_tuples(tags.iter(), |mut b, tag| {
          b.push_bind(tag.clone());
        });
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

    transaction.commit().await?;

    Ok(key_result.rows_affected() > 0)
  }

  pub async fn insert_key(&self, item: KeyData) -> Result<i64, Error> {
    let now: i64 = get_unix_timestamp()?;
    let mut conn = self.pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    let key_insert = query!(
      "INSERT INTO keys
        (pinned, target_size, revision, charset, domain, username, notes, created_at, custom_icon, version) VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      item.pinned,
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

    transaction.commit().await?;

    Ok(key_id)
  }

  pub async fn update_key(&self, key_id: i64, item: KeyData) -> Result<(), Error> {
    let mut conn = self.pool.acquire().await?;
    let mut transaction = conn.begin().await?;
    query!(
      "UPDATE keys SET
        (pinned, target_size, revision, charset, domain, username, notes, custom_icon, version) =
        (?, ?, ?, ?, ?, ?, ?, ?, ?)
      WHERE keys.id = ?;",
      item.pinned,
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
    let to_add = item.tags.difference(&existing_tags);
    let to_delete = existing_tags.difference(&item.tags);

    if to_delete.len() > 0 {
      let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("DELETE FROM tags WHERE (key_id, name) IN ");

      query_builder.push_tuples(to_delete.iter(), |mut b, tag| {
        b.push_bind(key_id);
        b.push_bind(tag.as_ref());
      });

      let query = query_builder.build();
      query.execute(&mut *transaction).await?;
    }

    if to_add.len() > 0 {
      let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO tags (key_id, name) ");

      query_builder.push_values(to_add.iter(), |mut b, tag| {
        b.push_bind(key_id);
        b.push_bind(tag.as_ref());
      });

      let query = query_builder.build();
      query.execute(&mut *transaction).await?;
    }

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
}
