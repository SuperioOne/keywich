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

  pub fn difference(&self, other: &TagList) -> Self {
    let diff: HashSet<Box<str>> = self
      .internal
      .difference(&other.internal)
      .map(|e| e.clone())
      .collect();

    TagList { internal: diff }
  }

  pub fn union(&self, other: &TagList) -> Self {
    let union: HashSet<Box<str>> = self
      .internal
      .union(&other.internal)
      .map(|e| e.clone())
      .collect();

    TagList { internal: union }
  }

  pub fn len(&self) -> usize {
    self.internal.len()
  }

  pub fn is_empty(&self) -> bool {
    self.internal.is_empty()
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

  pub fn join(&self, separator: char) -> String {
    let mut text = String::new();

    for (idx, tag) in self.internal.iter().enumerate() {
      text.push_str(tag);

      if idx != self.internal.len() - 1 {
        text.push(separator)
      }
    }

    return text;
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
