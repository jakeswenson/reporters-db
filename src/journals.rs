use std::collections::HashMap;
pub use chrono::NaiveDateTime;
use serde::{Deserialize};
use crate::regexes::RegexTemplate;

#[derive(Deserialize, Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JournalCiteType {
  Journal,
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct JournalAbbreviation(String);

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct JournalName(String);

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Journal {
  cite_type: JournalCiteType,
  start: Option<NaiveDateTime>,
  end: Option<NaiveDateTime>,
  examples: Vec<String>,
  name: JournalName,
  regexes: Vec<RegexTemplate>,
  notes: Option<String>,
  href: Option<String>,
}

pub type JournalsMap = HashMap<JournalAbbreviation, Vec<Journal>>;

pub fn journals() -> JournalsMap {
  let json = include_str!("../reporters_db/data/journals.json");
  serde_json::from_str(json)
    .expect("Parsing journals.json should not fail...")
}

#[cfg(test)]
mod tests {
  use super::journals;

  #[test]
  fn parse_journals() {
    dbg!(journals());
  }
}
