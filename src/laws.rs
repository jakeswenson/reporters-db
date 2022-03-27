use std::collections::HashMap;
pub use chrono::NaiveDateTime;
use serde::{Deserialize};
use crate::regexes::RegexTemplate;

#[derive(Deserialize, Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LawCiteType {
  AdminFiling,
  AdminRegister,
  AdminDocket,
  AdminCompilation,
  LegSession,
  LegStatute,
  Municipal,
  LegAct,
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LawAbbreviation(String);

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LawName(String);

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Jurisdiction(String);

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Law {
  cite_type: LawCiteType,
  start: Option<NaiveDateTime>,
  end: Option<NaiveDateTime>,
  examples: Vec<String>,
  jurisdiction: Jurisdiction,
  name: LawName,
  regexes: Vec<RegexTemplate>,
  notes: Option<String>,
  href: Option<String>,
}

pub type LawsMap = HashMap<LawAbbreviation, Vec<Law>>;

pub fn laws() -> LawsMap {
  let json = include_str!("../reporters_db/data/laws.json");
  serde_json::from_str(json)
    .expect("Parsing laws.json should not fail...")
}

#[cfg(test)]
mod tests {
  use super::laws;

  #[test]
  fn parse_laws() {
    dbg!(laws());
  }
}
