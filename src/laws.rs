// Re-use the Law type from generated code (uses static references for zero-copy access)
pub use crate::generated::Law;

// Type alias for compatibility
pub type LawsMap = phf::Map<&'static str, &'static [Law]>;

// Compatibility enums and structs for existing API
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct LawCiteType(&'static str);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LawAbbreviation(pub String);

impl LawAbbreviation {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LawName(pub String);

impl LawName {
  pub fn value(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Jurisdiction(pub String);

impl Jurisdiction {
  pub fn value(&self) -> &str {
    &self.0
  }
}

// Function to return the static LAWS map
pub fn laws() -> &'static LawsMap {
  &crate::generated::LAWS
}

#[cfg(test)]
mod tests {
  use super::laws;

  #[test]
  fn parse_laws() {
    let laws = laws();
    assert!(!laws.is_empty());
  }
}
