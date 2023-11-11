use sqlx::{Connection, SqliteConnection, query, query_as, FromRow};

#[derive(Debug, FromRow)]
pub struct TestItem
{
  pub id: i64,
  pub profile: String,
  pub domain: String,
  pub target_size: u8,
  pub charset: String,
}

pub async fn test() -> Result<(), sqlx::Error>
{
  println!("Creating memory database.");
  let mut connection = SqliteConnection::connect("sqlite::memory:").await?;

  _ = query("CREATE TABLE IF NOT EXISTS MyDudes (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      profile TEXT,
      domain TEXT,
      target_size INTEGER,
      charset TEXT,
      name TEXT
    ); ")
    .execute(&mut connection).await?;

  println!("Table Created.");
  _ = query("INSERT INTO MyDudes (id, profile, name, domain, target_size, charset) VALUES (1, 'profile_1', 'name_1', 'domain_1', 24, 'a..zA..Z')").execute(&mut connection).await?;
  _ = query("INSERT INTO MyDudes (id, profile, name, domain, target_size, charset) VALUES (2, 'profile_2', 'name_2', 'domain_2', 24, 'a..zA..Z')").execute(&mut connection).await?;
  _ = query("INSERT INTO MyDudes (id, profile, name, domain, target_size, charset) VALUES (3, 'profile_3', 'name_3', 'domain_3', 24, 'a..zA..Z')").execute(&mut connection).await?;
  println!("Inserted new entries.");

  let get_query = query_as::<_, TestItem>("SELECT * FROM MyDudes")
    .fetch_all(&mut connection).await?;

  println!("{:?}", get_query);

  Ok(())
}