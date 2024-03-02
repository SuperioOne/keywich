#[cfg(all(test, feature = "profile"))]
mod tests {
  use keywich_lib::profile::keys::{KeyData, SearchQuery};
  use keywich_lib::profile::utils::tag_list::TagList;
  use keywich_lib::profile::ProfileDB;

  macro_rules! generate_key {
    () => {
      keywich_lib::profile::keys::KeyData {
        notes: Some("notes".into()),
        domain: "domain".into(),
        version: "v1".into(),
        custom_icon: Some("/tmp/icon.ico".into()),
        username: "username".into(),
        charset: "a..z0..9".into(),
        revision: 12,
        target_size: 12,
        tags: TagList::from(["tag1", "tag2", "tag3", "tag4", "tag5", "tag6"]),
      }
    };
  }

  #[tokio::test]
  async fn read_key() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db.get_keys(false).await.unwrap();

    assert_eq!(2, keys.len());
  }

  #[tokio::test]
  async fn invalid_data() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let key_data = KeyData {
      notes: Some("notes".into()),
      domain: "".into(),
      version: "v1".into(),
      custom_icon: Some("/tmp/icon.ico".into()),
      username: "".into(),
      charset: "a..0..9".into(),
      revision: 12,
      target_size: 900,
      tags: TagList::from(["tag1", "tag2", "tag3", "tag4", "tag5", "tag6"]),
    };

    if let Err(keywich_lib::errors::Error::ValidationError(errors)) =
      profile_db.insert_key(key_data).await
    {
      assert_eq!(4, errors.errors().len())
    } else {
      assert!(false)
    }
  }

  #[tokio::test]
  async fn empty_search() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db.search_keys("".into()).await.unwrap();

    assert_eq!(3, keys.len());
  }

  #[tokio::test]
  async fn search_by_single_param() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db
      .search_keys("tag:tag1 tag:tag2".into())
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_two_params() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db
      .search_keys("tag:tag1 tag:tag2 domain:domain".into())
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_all_params() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db
      .search_keys("tag:tag1 tag:tag2 domain:domain username:username".into())
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn delete_key() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let key_id = profile_db.insert_key(generate_key!()).await.unwrap();

    profile_db.delete_key(key_id).await.unwrap();

    assert!(true);
  }

  #[tokio::test]
  async fn update_key() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let key_id = profile_db.insert_key(generate_key!()).await.unwrap();

    profile_db
      .update_key(
        key_id,
        KeyData {
          notes: Some("notes".into()),
          domain: "domain".into(),
          version: "v2".into(),
          custom_icon: Some("/tmp/icon.ico".into()),
          username: "username".into(),
          charset: "a..z0..9".into(),
          revision: 13,
          target_size: 13,
          tags: TagList::from(["tag4", "tag5", "tag6", "tag7", "tag8"]),
        },
      )
      .await
      .unwrap();

    assert!(true);
  }

  #[tokio::test]
  async fn get_by_id() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let key_id = profile_db.insert_key(generate_key!()).await.unwrap();

    let result = profile_db.get_key_by_id(key_id).await.unwrap();

    assert_eq!(true, result.is_some());
    assert_eq!(key_id, result.unwrap().id);
  }

  #[tokio::test]
  async fn get_non_existing_by_id() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let result = profile_db.get_key_by_id(999).await.unwrap();

    assert_eq!(true, result.is_none())
  }

  #[tokio::test]
  async fn fts_query() {
    let query =
      SearchQuery::new("tag:tag1 tag:tag2 domain:test username:user\" test query like a tag");
    if let Some(qtext) = query.to_fts_query() {
      assert_eq!("({domain username tags notes}: \"test\" OR \"query\" OR \"like\" OR \"a\" OR \"tag\") AND ({username}: \"user\"\"\") AND ({domain}: \"test\") AND ({tags}: \"tag1\" OR \"tag2\")", &qtext)
    } else {
      assert!(false);
    }
  }
}
