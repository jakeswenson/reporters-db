pub use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Captures;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RegexTemplate(String);

lazy_static! {
    static ref RESOLVER: regex::Regex = regex::Regex::new(r"\$(\w+)").unwrap();
}

impl RegexTemplate {
    pub fn of<T: Into<String>>(value: T) -> Self {
        Self(value.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn resolve(&self, values: &HashMap<String, RegexTemplate>) -> Self {
        let new_value = RESOLVER.replace_all(&self.0, |caps: &Captures| {
            let template = values
                .get(&caps[1])
                .map(|t| t.0.as_str())
                .unwrap_or(&caps[0]);

            template.to_string()
        });

        Self(new_value.to_string())
    }

    pub fn resolved(self) -> Result<ResolvedRegex, UnresolvedRegex> {
        if RESOLVER.is_match(&self.0) {
            Err(UnresolvedRegex(self))
        } else {
            Ok(ResolvedRegex::of(self.0))
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, thiserror::Error)]
#[error("Unresolved regex: {0:?}")]
pub struct UnresolvedRegex(RegexTemplate);

impl UnresolvedRegex {
    pub fn template(self) -> RegexTemplate {
        self.0
    }

    pub fn template_ref(&self) -> &RegexTemplate {
        &self.0
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
    pub fn of(value: String) -> Self {
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

impl RegexOrNested {
    pub fn add<T: Into<String>>(&mut self, key: T, value: RegexTemplate) {
        match self {
            RegexOrNested::Nested(map) => {
                map.insert(key.into(), RegexOrNested::Regex(value));
            }
            RegexOrNested::Regex(_) => panic!("Can't added a regex to a non nested namespace"),
        }
    }
}

pub fn raw_regexes() -> RawRegexMap {
    let json = include_str!("../reporters_db/data/regexes.json");
    serde_json::from_str(json).expect("Parsing regexes.json should not fail...")
}

pub fn regexes() -> HashMap<String, RegexTemplate> {
    crate::utils::process_variables(raw_regexes())
}

#[cfg(test)]
mod tests {
    use super::regexes;
    use crate::regexes::RegexTemplate;

    #[test]
    fn parse_regexes() {
        dbg!(regexes());
    }

    #[test]
    fn resolve_regex() {
        let template = RegexTemplate::of("$edition").resolve(
            &vec![("edition".into(), RegexTemplate::of("foo"))]
                .into_iter()
                .collect(),
        );

        assert_eq!(template, RegexTemplate::of("foo"));
    }
}
