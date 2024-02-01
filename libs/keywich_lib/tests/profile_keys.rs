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
        pinned: false,
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
  async fn search_by_single_param() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db
      .search_keys(SearchQuery {
        tag: Some(["tag1", "tag2"].into()),
        domain: None,
        username: None,
      })
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_two_params() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db
      .search_keys(SearchQuery {
        tag: Some(["tag1", "tag2"].into()),
        domain: Some(vec![String::from("domain")]),
        username: None,
      })
      .await
      .unwrap();

    assert!(!keys.is_empty());
  }

  #[tokio::test]
  async fn search_by_all_params() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    profile_db.insert_key(generate_key!()).await.unwrap();

    let keys = profile_db
      .search_keys(SearchQuery {
        tag: Some(["tag1", "tag2"].into()),
        domain: Some(vec![String::from("domain")]),
        username: Some(vec![String::from("username")]),
      })
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
          pinned: true,
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
}
