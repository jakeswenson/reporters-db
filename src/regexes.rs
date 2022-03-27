use std::collections::HashMap;
pub use chrono::NaiveDateTime;
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RegexTemplate(pub String);

pub(crate) type RawRegexMap = HashMap<String, RegexOrNested>;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum RegexOrNested {
  Regex(RegexTemplate),
  Nested(RawRegexMap)
}

fn raw_regexes() -> RawRegexMap {
  let json = include_str!("../reporters_db/data/regexes.json");
  serde_json::from_str(json)
    .expect("Parsing regexes.json should not fail...")
}

pub fn regexes() -> HashMap<String, RegexTemplate> {
  crate::utils::process_variables(raw_regexes())
}

#[cfg(test)]
mod tests {
  use super::regexes;

  #[test]
  fn parse_regexes() {
    dbg!(regexes());
  }
}
