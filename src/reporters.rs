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

impl EditionName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Edition {
  pub end: Option<NaiveDateTime>,
  pub start: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReporterName(String);

impl ReporterName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Reporter {
  pub cite_type: CiteType,
  pub editions: HashMap<EditionName, Edition>,
  pub mlz_jurisdiction: Vec<String>,
  pub name: ReporterName,
  pub variations: HashMap<EditionName, EditionName>,
  pub href: Option<String>,
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
