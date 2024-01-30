use crate::errors::Error;
use crate::profile::utils::tag_list::TagList;
use crate::profile::utils::timestamp::get_unix_timestamp;
use serde::Serialize;
use sqlx::{query, query_as, Connection, FromRow, QueryBuilder, Sqlite, SqliteConnection};

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
pub struct KeyItemRow {
  pub id: i64,
  pub pinned: bool,
  pub target_size: i64,
  pub revision: i64,
  pub charset: String,
  pub domain: String,
  pub user_name: String,
  pub notes: Option<String>,
  pub created_at: i64,
  pub custom_icon: Option<String>,
  pub version: String,

  // Default value must be an empty array for tags since current sqlx version does not support nullable json columns.
  // See https://github.com/launchbadge/sqlx/issues/2849
  #[sqlx(json)]
  pub tags: TagList,
}

#[derive(Debug, Serialize)]
pub struct KeyData {
  pub pinned: bool,
  pub target_size: i64,
  pub revision: i64,
  pub charset: String,
  pub domain: String,
  pub user_name: String,
  pub notes: Option<String>,
  pub custom_icon: Option<String>,
  pub version: String,
  pub tags: TagList,
}

pub struct SearchQuery {
  user_name: Option<Vec<String>>,
  domain: Option<Vec<String>>,
  tag: Option<TagList>,
}

pub struct Keys {
  connection: SqliteConnection,
}

impl Keys {
  pub fn new(connection: SqliteConnection) -> Self {
    Self { connection }
  }

  pub async fn get_key_by_id(&mut self, key_id: i64) -> Result<Option<KeyItemRow>, Error> {
    let row = query_as::<Sqlite, KeyItemRow>(
      "SELECT
        keys.id,
        keys.pinned,
        keys.target_size,
        keys.revision,
        keys.charset,
        keys.domain,
        keys.user_name,
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
    .fetch_one(&mut self.connection)
    .await;

    match row {
      Ok(result) => Ok(Some(result)),
      Err(sqlx::Error::RowNotFound) => Ok(None),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn get_keys(&mut self, pinned_only: bool) -> Result<Vec<KeyItemRow>, Error> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
      "SELECT
        keys.id,
        keys.pinned,
        keys.target_size,
        keys.revision,
        keys.charset,
        keys.domain,
        keys.user_name,
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

    let query = query_builder.build_query_as::<KeyItemRow>();
    let result: Vec<KeyItemRow> = query.fetch_all(&mut self.connection).await?;

    Ok(result)
  }

  pub async fn search_keys(&mut self, search_query: SearchQuery) -> Result<Vec<KeyItemRow>, Error> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
      "SELECT DISTINCT
        keys.id,
        keys.pinned,
        keys.target_size,
        keys.revision,
        keys.charset,
        keys.domain,
        keys.user_name,
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

    if let Some(user_names) = search_query.user_name {
      insert_token!(&mut query_builder, &mut insert_where, " WHERE ");
      query_builder
        .push(" keys.user_name IN ")
        .push_tuples(user_names, |mut b, username| {
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

    let query = query_builder.build_query_as::<KeyItemRow>();
    let result: Vec<KeyItemRow> = query.fetch_all(&mut self.connection).await?;

    Ok(result)
  }

  pub async fn delete_key(&mut self, key_id: i64) -> Result<bool, Error> {
    let mut transaction = self.connection.begin().await?;

    query!("DELETE FROM tags WHERE tags.key_id = ?", key_id)
      .execute(&mut *transaction)
      .await?;

    let key_result = query!("DELETE FROM keys WHERE keys.id = ?", key_id)
      .execute(&mut *transaction)
      .await?;

    transaction.commit().await?;

    Ok(key_result.rows_affected() > 0)
  }

  pub async fn insert_key(&mut self, item: KeyData) -> Result<i64, Error> {
    let now: i64 = get_unix_timestamp()?;
    let mut transaction = self.connection.begin().await?;
    let key_insert = query!(
      "INSERT INTO keys
        (pinned, target_size, revision, charset, domain, user_name, notes, created_at, custom_icon, version) VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      item.pinned,
      item.target_size,
      item.revision,
      item.charset,
      item.domain,
      item.user_name,
      item.notes,
      now,
      item.custom_icon,
      item.version
    )
    .execute(&mut *transaction)
    .await?;

    let key_id = key_insert.last_insert_rowid();
    let mut query_builder: QueryBuilder<Sqlite> =
      QueryBuilder::new("INSERT INTO tags (name, key_id) ");

    query_builder.push_values(item.tags.iter(), |mut b, tag| {
      b.push_bind(tag.as_ref());
      b.push_bind(key_id);
    });

    let query = query_builder.build();

    query.execute(&mut *transaction).await?;
    transaction.commit().await?;

    Ok(key_id)
  }

  pub async fn update_key(&mut self, key_id: i64, item: KeyData) -> Result<(), Error> {
    let mut transaction = self.connection.begin().await?;
    query!(
      "UPDATE keys SET
        (pinned, target_size, revision, charset, domain, user_name, notes, custom_icon, version) =
        (?, ?, ?, ?, ?, ?, ?, ?, ?)
      WHERE keys.id = ?;",
      item.pinned,
      item.target_size,
      item.revision,
      item.charset,
      item.domain,
      item.user_name,
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

  pub async fn update_pin_status(&mut self, key_id: i64, pin_status: bool) -> Result<(), Error> {
    query!(
      "UPDATE keys SET pinned = ? WHERE keys.id = ?;",
      pin_status,
      key_id
    )
    .execute(&mut self.connection)
    .await?;

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::profile::keys::{KeyData, Keys, SearchQuery};
  use crate::profile::utils::tag_list::TagList;
  use sqlx::Connection;
  use sqlx::SqliteConnection;

  #[tokio::test]
  async fn read_test() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);
    let keys = key_api.get_keys(false).await.unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_single_param() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);
    let keys = key_api
      .search_keys(SearchQuery {
        tag: Some(["tag1", "tag2"].into()),
        domain: None,
        user_name: None,
      })
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_two_params() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);
    let keys = key_api
      .search_keys(SearchQuery {
        tag: Some(["tag1", "tag2"].into()),
        domain: Some(vec![String::from("dasd")]),
        user_name: None,
      })
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_all_params() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);
    let keys = key_api
      .search_keys(SearchQuery {
        tag: Some(["tag1", "tag2"].into()),
        domain: Some(vec![String::from("dasd")]),
        user_name: Some(vec![String::from("test")]),
      })
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn insert_update_delete_test() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);

    let key_id = key_api
      .insert_key(KeyData {
        notes: Some("notes".into()),
        domain: "domain".into(),
        version: "v1".into(),
        custom_icon: Some("/tmp/icon.ico".into()),
        user_name: "username".into(),
        charset: "a..z0..9".into(),
        revision: 12,
        target_size: 12,
        pinned: false,
        tags: TagList::from(["tag1", "tag2", "tag3", "tag4", "tag5", "tag6"]),
      })
      .await
      .unwrap();

    key_api
      .update_key(
        key_id,
        KeyData {
          notes: Some("notes".into()),
          domain: "domain".into(),
          version: "v2".into(),
          custom_icon: Some("/tmp/icon.ico".into()),
          user_name: "username".into(),
          charset: "a..z0..9".into(),
          revision: 13,
          target_size: 13,
          pinned: true,
          tags: TagList::from(["tag4", "tag5", "tag6", "tag7", "tag8"]),
        },
      )
      .await
      .unwrap();

    key_api.delete_key(key_id).await.unwrap();

    assert!(true);
  }

  #[tokio::test]
  async fn get_by_id() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);
    let result = key_api.get_key_by_id(1).await.unwrap();

    assert_eq!(true, result.is_some());
  }

  #[tokio::test]
  async fn get_non_existing_by_id() {
    let sqlite_connection = SqliteConnection::connect("sqlite:dev.sqlite")
      .await
      .unwrap();

    let mut key_api = Keys::new(sqlite_connection);
    let result = key_api.get_key_by_id(-1).await.unwrap();

    assert_eq!(true, result.is_none())
  }
}
