use crate::regexes::RegexTemplate;
pub use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;

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

impl LawAbbreviation {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LawName(String);

impl LawName {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Jurisdiction(String);

impl Jurisdiction {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Law {
    pub cite_type: LawCiteType,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub examples: Vec<String>,
    pub jurisdiction: Jurisdiction,
    pub name: LawName,
    pub regexes: Vec<RegexTemplate>,
    pub notes: Option<String>,
    pub href: Option<String>,
}

pub type LawsMap = HashMap<LawAbbreviation, Vec<Law>>;

pub fn laws() -> LawsMap {
    let json = include_str!("../reporters_db/data/laws.json");
    serde_json::from_str(json).expect("Parsing laws.json should not fail...")
}

#[cfg(test)]
mod tests {
    use super::laws;

    #[test]
    fn parse_laws() {
        dbg!(laws());
    }
}
