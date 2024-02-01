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
    profile_db
      .insert_charset(generate_charset!())
      .await
      .unwrap();

    let charset_items = profile_db.get_charsets().await.unwrap();

    assert_eq!(1, charset_items.len())
  }

  #[tokio::test]
  async fn delete_charset() {
    let profile_db = ProfileDB::connect("sqlite::memory:").await.unwrap();

    let inserted = profile_db
      .insert_charset(generate_charset!())
      .await
      .unwrap();

    assert_eq!(1, inserted);

    let deleted_row = profile_db.delete_charset("__test").await.unwrap();

    assert_eq!(1, deleted_row);
  }
}
