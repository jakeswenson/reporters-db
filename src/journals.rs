use crate::regexes::RegexTemplate;
pub use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JournalCiteType {
    Journal,
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct JournalAbbreviation(String);

impl JournalAbbreviation {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct JournalName(String);

impl JournalName {
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Journal {
    pub cite_type: JournalCiteType,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub examples: Vec<String>,
    pub name: JournalName,
    pub regexes: Vec<RegexTemplate>,
    pub notes: Option<String>,
    pub href: Option<String>,
}

pub type JournalsMap = HashMap<JournalAbbreviation, Vec<Journal>>;

pub fn journals() -> JournalsMap {
    let json = include_str!("../reporters_db/data/journals.json");
    serde_json::from_str(json).expect("Parsing journals.json should not fail...")
}

#[cfg(test)]
mod tests {
    use super::journals;

    #[test]
    fn parse_journals() {
        dbg!(journals());
    }
}
