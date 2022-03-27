use std::collections::HashMap;
pub use chrono::NaiveDateTime;
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CiteType {
  StateRegional,
  State,
  Federal,
  Specialty,
  Neutral,
  SpecialtyLexis,
  ScotusEarly,
  SpecialtyWest,
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EditionName(String);

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Edition {
  end: Option<NaiveDateTime>,
  start: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReporterName(String);

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Reporter {
  cite_type: CiteType,
  editions: HashMap<EditionName, Edition>,
  mlz_jurisdiction: Vec<String>,
  name: ReporterName,
  variations: HashMap<EditionName, EditionName>,
  href: Option<String>,
}

pub type ReportersMap = HashMap<EditionName, Vec<Reporter>>;

pub fn reporters() -> ReportersMap {
  let reporters = include_str!("../reporters_db/data/reporters.json");
  serde_json::from_str(reporters)
    .expect("Parsing reporters.json should not fail...")
}

#[cfg(test)]
mod tests {
  use super::reporters;

  #[test]
  fn parse_reporters() {
    dbg!(reporters());
  }
}
