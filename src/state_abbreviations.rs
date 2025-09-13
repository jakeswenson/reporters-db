// Type alias for compatibility
pub type StateAbbreviationMap = phf::Map<&'static str, &'static str>;

// Compatibility struct wrappers for existing API
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StateAbbreviation(pub String);

impl StateAbbreviation {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StateName(pub String);

impl StateName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

// Function to return the static STATE_ABBREVIATIONS map
pub fn state_abbreviations() -> &'static StateAbbreviationMap {
  &crate::generated::STATE_ABBREVIATIONS
}

#[cfg(test)]
mod tests {
  use super::state_abbreviations;

  #[test]
  fn parse_state_abbreviations() {
    let abbreviations = state_abbreviations();
    assert!(!abbreviations.is_empty());
  }
}
