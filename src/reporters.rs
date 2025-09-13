// Include the generated PHF data
include!(concat!(env!("OUT_DIR"), "/generated_data.rs"));

// Type alias for compatibility
pub type ReportersMap = phf::Map<&'static str, &'static [Reporter]>;

// Compatibility struct wrappers for existing API
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EditionName(pub String);

impl EditionName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReporterName(pub String);

impl ReporterName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

// Function to return the static REPORTERS map
pub fn reporters() -> &'static ReportersMap {
  &REPORTERS
}

#[cfg(test)]
mod tests {
  use super::reporters;

  #[test]
  fn parse_reporters() {
    let reporters = reporters();
    assert!(!reporters.is_empty());
    // Test a known reporter
    assert!(reporters.get("A.").is_some());
  }
}
