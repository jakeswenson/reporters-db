use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CaseNamePartAbbreviation(String);

impl CaseNamePartAbbreviation {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CaseNamePart(String);

impl CaseNamePart {
  pub fn value(&self) -> &str {
    &self.0
  }
}

pub type CaseNamePartAbbreviationMap = HashMap<CaseNamePartAbbreviation, Vec<CaseNamePart>>;

pub fn case_name_part_abbreviations() -> CaseNamePartAbbreviationMap {
  let json = include_str!("../reporters_db/data/case_name_abbreviations.json");
  serde_json::from_str(json)
    .expect("Parsing case_name_abbreviations.json should not fail...")
}

#[cfg(test)]
mod tests {
  use super::case_name_part_abbreviations;

  #[test]
  fn parse_case_name_part_abbreviations() {
    dbg!(case_name_part_abbreviations());
  }
}
