#[cfg(all(test, feature = "profile"))]
mod test {
  use keywich_lib::profile::utils::tag_list::TagList;
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

    assert_eq!(3, tags_str.len());
    assert_eq!(3, tags_string.len());
    assert_eq!(3, tags_set.len());
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
    assert_eq!(3, result.unwrap().len())
  }

  #[test]
  fn union() {
    let tag_list_a = TagList::from(["str_tag1", "str_tag2", "str_tag3"]);
    let tag_list_b = TagList::from(["str_tag1", "str_tag2", "str_tag4"]);
    let union_list = tag_list_a.union(&tag_list_b);

    assert_eq!(4, union_list.len());
  }

  #[test]
  fn difference() {
    let tag_list_a = TagList::from(["str_tag1", "str_tag2", "str_tag3"]);
    let tag_list_b = TagList::from(["str_tag1", "str_tag2", "str_tag4"]);
    let diff_list = tag_list_a.difference(&tag_list_b);

    assert_eq!(1, diff_list.len());
  }
}
