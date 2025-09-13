// Re-use the Journal type from generated code (uses static references for zero-copy access)
pub use crate::generated::Journal;

// Type alias for compatibility
pub type JournalsMap = phf::Map<&'static str, &'static [Journal]>;

// Compatibility struct wrappers for existing API
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct JournalCiteType(&'static str);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct JournalAbbreviation(pub String);

impl JournalAbbreviation {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct JournalName(pub String);

impl JournalName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

// Function to return the static JOURNALS map
pub fn journals() -> &'static JournalsMap {
  &crate::generated::JOURNALS
}

#[cfg(test)]
mod tests {
  use super::journals;

  #[test]
  fn parse_journals() {
    let journals = journals();
    assert!(!journals.is_empty());
  }
}
