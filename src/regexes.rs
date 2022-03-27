pub use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Captures;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RegexTemplate(String);

lazy_static! {
    static ref RESOLVER: regex::Regex = regex::Regex::new(r"$(\w+)").unwrap();
}

impl RegexTemplate {
    pub fn of(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn resolve(&self, values: &HashMap<String, RegexTemplate>) -> Self {
        let new_value = RESOLVER.replace_all(&self.0, |caps: &Captures| {
            let template = values
                .get(&caps[1])
                .expect("Unable to resolve regex template replacement value");

            &template.0
        });

        Self(new_value.to_string())
    }
}

impl From<RegexTemplate> for String {
    fn from(template: RegexTemplate) -> Self {
        template.0
    }
}

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ResolvedRegex(String);

impl ResolvedRegex {
    pub(crate) fn of(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub(crate) type RawRegexMap = HashMap<String, RegexOrNested>;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum RegexOrNested {
    Regex(RegexTemplate),
    Nested(RawRegexMap),
}

pub fn raw_regexes() -> RawRegexMap {
    let json = include_str!("../reporters_db/data/regexes.json");
    serde_json::from_str(json).expect("Parsing regexes.json should not fail...")
}

pub fn regexes() -> HashMap<String, ResolvedRegex> {
    crate::utils::process_variables(raw_regexes()).expect("Failed to resolve regexs")
}

#[cfg(test)]
mod tests {
    use super::regexes;

    #[test]
    fn parse_regexes() {
        dbg!(regexes());
    }
}
