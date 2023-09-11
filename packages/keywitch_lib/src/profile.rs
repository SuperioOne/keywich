use crate::errors::KeywitchError;
use crate::profile::models::{CharsetItem, PassMetadataItem};

pub mod storage;
pub mod models;

pub fn get_pinned_pass_collection() -> Result<Vec<PassMetadataItem>, KeywitchError>
{
// TODO: emulating endpoints for now
  let vec: Vec<PassMetadataItem> = get_pass_metadata_collection()?
    .into_iter()
    .filter(|e| e.pinned == true)
    .collect();

  Ok(vec)
}

pub fn get_charset_collection() -> Result<Vec<CharsetItem>, KeywitchError>
{
  // TODO: emulating endpoints for now
  let mut vec: Vec<CharsetItem> = vec![];

  for i in 0..5 {
    vec.push(CharsetItem {
      charset: String::from("A..Za..z0..9"),
      description: Some(String::from(format!("User {}", i))),
      id: i,
    });
  }

  Ok(vec)
}

pub fn get_pass_metadata_collection() -> Result<Vec<PassMetadataItem>, KeywitchError>
{
  // TODO: emulating endpoints for now
  let mut vec: Vec<PassMetadataItem> = vec![];

  for i in 0..15 {
    vec.push(PassMetadataItem {
      charset: String::from("A..Za..z0..9"),
      user_name: String::from(format!("User {}", i)),
      notes: Some(String::from("example note")),
      domain: String::from("example.com"),
      revision: 0,
      target_size: 36,
      id: i,
      pinned: i % 5 == 0,
    });
  }

  Ok(vec)
}