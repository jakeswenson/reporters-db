//! Legal court reporters database module.
//!
//! This module provides access to the main court reporters database, including
//! reporter metadata, editions, variations, and citation types. All data uses
//! static references for zero-copy access at runtime.
//!
//! # Examples
//!
//! ```rust
//! use reporters_db::reporters::{reporters, CiteType};
//!
//! let reporters_db = reporters();
//! if let Some(reporter_list) = reporters_db.get("F.3d") {
//!     for reporter in reporter_list.iter() {
//!         println!("Reporter: {}", reporter.name);
//!         match reporter.cite_type {
//!             CiteType::Federal => println!("This is a federal reporter"),
//!             CiteType::State => println!("This is a state reporter"),
//!             _ => println!("Other citation type: {:?}", reporter.cite_type),
//!         }
//!     }
//! }
//! ```

// Use the generated PHF data
use crate::generated::REPORTERS;

// Re-export the generated types for public API (these use static references for zero-copy access)
pub use crate::generated::{Edition, Reporter};
// Re-export CiteType from types module (shared between parsing and runtime)
pub use crate::types::CiteType;

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

/// Returns the main court reporters database.
///
/// This function provides access to the complete database of legal court reporters,
/// indexed by their abbreviation keys. Each key maps to a list of [`Reporter`] structs
/// containing detailed information about editions, variations, jurisdictions, and more.
///
/// # Returns
///
/// A static reference to the reporters database that lives for the entire program duration.
/// The data is embedded at compile time for optimal performance.
///
/// # Examples
///
/// ```rust
/// use reporters_db::reporters::reporters;
///
/// let db = reporters();
///
/// // Get reporters for a specific abbreviation
/// if let Some(reporter_list) = db.get("U.S.") {
///     for reporter in reporter_list.iter() {
///         println!("Reporter: {}", reporter.name);
///         println!("Jurisdictions: {:?}", reporter.mlz_jurisdiction);
///     }
/// }
/// ```
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
