// Type alias for compatibility
pub type CaseNamePartAbbreviationMap = phf::Map<&'static str, &'static [&'static str]>;

// Compatibility struct wrappers for existing API
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CaseNamePartAbbreviation(pub String);

impl CaseNamePartAbbreviation {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CaseNamePart(pub String);

impl CaseNamePart {
  pub fn value(&self) -> &str {
    &self.0
  }
}

// Function to return the static CASE_NAME_ABBREVIATIONS map
pub fn case_name_part_abbreviations() -> &'static CaseNamePartAbbreviationMap {
  &crate::reporters::CASE_NAME_ABBREVIATIONS
}

#[cfg(test)]
mod tests {
  use super::case_name_part_abbreviations;

  #[test]
  fn parse_case_name_part_abbreviations() {
    let abbreviations = case_name_part_abbreviations();
    assert!(!abbreviations.is_empty());
  }
}
