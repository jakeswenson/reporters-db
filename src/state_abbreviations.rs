use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StateAbbreviation(String);

impl StateAbbreviation {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StateName(String);

impl StateName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

pub type StateAbbreviationMap = HashMap<StateAbbreviation, StateName>;

pub fn state_abbreviations() -> StateAbbreviationMap {
  let json = include_str!("../reporters_db/data/state_abbreviations.json");
  serde_json::from_str(json)
    .expect("Parsing state_abbreviations.json should not fail...")
}

#[cfg(test)]
mod tests {
  use super::state_abbreviations;

  #[test]
  fn parse_state_abbreviations() {
    dbg!(state_abbreviations());
  }
}
