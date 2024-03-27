#[cfg(all(test, feature = "profile"))]
mod tests {
  use keywich_lib::profile::charsets::CharsetItem;
  use keywich_lib::profile::ProfileDB;

  macro_rules! generate_charset {
    () => {
      CharsetItem {
        charset: "a..zA..Z".into(),
        description: Some("Description".into()),
        name: "__test".into(),
      }
    };
  }

  #[tokio::test]
  async fn read_charsets() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let charset_items = profile_db.get_charsets().await.unwrap();

    // By default 3 charset should be available.
    assert_eq!(3, charset_items.len())
  }

  #[tokio::test]
  async fn invalid_data() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();
    let charset = CharsetItem {
      charset: "a..A..Z".into(),
      description: None,
      name: "".into(),
    };

    if let Err(keywich_lib::errors::Error::ValidationError(details)) =
      profile_db.insert_charset(charset).await
    {
      assert_eq!(2, details.errors().len())
    } else {
      assert!(false)
    }
  }

  #[tokio::test]
  async fn delete_charset() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();

    let inserted = profile_db
      .insert_charset(generate_charset!())
      .await
      .unwrap();

    assert_eq!("__test", &inserted);

    let deleted_row = profile_db.delete_charset("__test").await.unwrap();

    assert_eq!(1, deleted_row);
  }
}
