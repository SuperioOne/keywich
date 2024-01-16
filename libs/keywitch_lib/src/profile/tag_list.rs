use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

pub struct TagList {
  internal: HashSet<Box<str>>,
}

impl TagList {
  pub fn new() -> Self {
    Self {
      internal: HashSet::new(),
    }
  }

  pub fn iter(&self) -> Iter<'_, Box<str>> {
    self.internal.iter()
  }

  pub fn difference<'a>(&mut self, other: TagList) -> Self {
    let diff: HashSet<Box<str>> = self
      .internal
      .difference(&other.internal)
      .map(|e| e.clone())
      .collect();

    TagList { internal: diff }
  }

  pub fn insert(&mut self, value: &str) -> bool {
    self.internal.insert(Box::from(value))
  }

  pub fn remove(&mut self, value: &str) -> bool {
    self.internal.remove(value)
  }

  pub fn has(&mut self, value: &str) -> bool {
    self.internal.contains(value)
  }
}

impl Debug for TagList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(format!("{:?}", &self.internal).as_str())
  }
}

impl<T, TInner> From<T> for TagList
where
  T: Sized + IntoIterator<Item = TInner>,
  TInner: Sized + AsRef<str>,
{
  fn from(value: T) -> Self {
    let internal: HashSet<Box<str>> = value.into_iter().map(|e| Box::from(e.as_ref())).collect();
    TagList { internal }
  }
}

impl<'de> Deserialize<'de> for TagList {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct TagVisitor;

    impl TagVisitor {
      fn new() -> Self {
        TagVisitor
      }
    }

    impl<'de> Visitor<'de> for TagVisitor {
      type Value = TagList;

      fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string array")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
      where
        A: SeqAccess<'de>,
      {
        let mut tag_list = TagList::new();
        while let Some(element) = seq.next_element::<String>()? {
          tag_list.insert(&element);
        }

        Ok(tag_list)
      }
    }

    deserializer.deserialize_seq(TagVisitor::new())
  }
}

impl Serialize for TagList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut sequence = serializer.serialize_seq(Some(self.internal.len()))?;
    for value in self.internal.iter() {
      sequence.serialize_element(value.as_ref())?;
    }
    sequence.end()
  }
}

#[cfg(test)]
mod test {
  use crate::profile::tag_list::TagList;
  use std::collections::HashSet;

  #[test]
  fn iterate() {
    let tags_str = TagList::from(["str_tag1", "str_tag2", "str_tag3"]);
    let tags_string = TagList::from(vec![
      String::from("string_tag1"),
      String::from("string_tag2"),
      String::from("string_tag3"),
    ]);
    let tags_set = TagList::from(HashSet::from(["hs_tag1", "hs_tag2", "hs_tag3"]));

    assert_eq!(3, tags_str.internal.len());
    assert_eq!(3, tags_string.internal.len());
    assert_eq!(3, tags_set.internal.len());
  }

  #[test]
  fn serialize() {
    let tags = TagList::from(["tag1", "tag2", "tag3"]);
    let result = serde_json::to_string(&tags);

    assert_eq!(true, result.is_ok())
  }

  #[test]
  fn deserialize() {
    let text = "[\"tag1\", \"tag2\", \"tag3\"]";
    let result = serde_json::from_str::<TagList>(text);

    assert_eq!(true, result.is_ok());
    assert_eq!(3, result.unwrap().internal.len())
  }
}
