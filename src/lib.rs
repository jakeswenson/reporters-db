//! # Reporters Database
//!
//! A Rust port of the Free Law Project's database of court reporters with zero runtime overhead.
//!
//! This library provides access to a comprehensive database of legal court reporters,
//! including their variations, editions, and citation formats. All data is embedded
//! at compile time using Perfect Hash Functions (PHF) for optimal performance.
//!
//! ## Quick Start
//!
//! ```rust
//! use reporters_db::{get_reporters, get_variations_only};
//!
//! // Get the main reporters database
//! let reporters = get_reporters();
//! if let Some(reporter_list) = reporters.get("A.2d") {
//!     println!("Found {} reporters for 'A.2d'", reporter_list.len());
//! }
//!
//! // Get variation mappings
//! let variations = get_variations_only();
//! if let Some(canonical_forms) = variations.get("Atlantic Reporter") {
//!     println!("Canonical forms: {:?}", canonical_forms);
//! }
//! ```
//!
//! ## Main API Functions
//!
//! - [`get_reporters()`] - Main reporters database
//! - [`get_variations_only()`] - Variation to canonical mappings
//! - [`get_editions()`] - Edition to reporter mappings
//! - [`get_names_to_editions()`] - Reporter names to edition abbreviations
//! - [`get_regex_variables()`] - Processed regex templates for citation parsing
//!
//! ## Performance
//!
//! This library uses compile-time Perfect Hash Functions (PHF) for all data access,
//! providing O(1) lookups with zero runtime overhead. All data is validated and
//! embedded during compilation.

#![deny(clippy::wildcard_imports)]

use std::collections::HashMap;

pub mod case_name_part_abbreviations;
pub(crate) mod generated;
pub mod journals;
pub mod laws;
pub mod regexes;
pub mod reporters;
pub mod state_abbreviations;
pub mod types;
pub mod utils;

#[cfg(test)]
mod validation_tests;

// Re-export key types for convenience
pub use case_name_part_abbreviations::CaseNamePartAbbreviationMap;
pub use journals::{Journal, JournalsMap};
pub use laws::{Law, LawsMap};
pub use regexes::{RegexTemplate, ResolvedRegex, UnresolvedRegex};
pub use reporters::{CiteType, Edition, Reporter, ReportersMap};
pub use state_abbreviations::StateAbbreviationMap;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
  #[error("Failed to resolve regex because there was too much recursion in the templates")]
  TooMuchRecursion(#[from] regexes::UnresolvedRegex),
}

// Public API functions that mirror the Python interface

/// Get the main reporters database containing legal court reporters and their metadata.
///
/// Returns a map from reporter abbreviation keys to lists of [`Reporter`] structs.
/// Each reporter contains information about editions, variations, jurisdictions, and citation types.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_reporters;
///
/// let reporters = get_reporters();
///
/// // Look up a specific reporter abbreviation
/// if let Some(reporter_list) = reporters.get("F.3d") {
///     for reporter in reporter_list.iter() {
///         println!("Reporter: {}", reporter.name);
///         println!("Cite type: {:?}", reporter.cite_type);
///         println!("Editions: {:?}", reporter.editions.keys().collect::<Vec<_>>());
///     }
/// }
/// ```
pub fn get_reporters() -> &'static ReportersMap {
  reporters::reporters()
}

/// Get state abbreviations mapping for US states.
///
/// Returns a map from state names to their common abbreviations used in legal citations.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_state_abbreviations;
///
/// let abbreviations = get_state_abbreviations();
/// if let Some(abbrev) = abbreviations.get("California") {
///     println!("California abbreviation: {}", abbrev);
/// }
/// ```
pub fn get_state_abbreviations() -> &'static StateAbbreviationMap {
  state_abbreviations::state_abbreviations()
}

/// Get case name part abbreviations used in legal case names.
///
/// Returns a map from full terms to their abbreviated forms commonly used in case titles.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_case_name_abbreviations;
///
/// let abbreviations = get_case_name_abbreviations();
/// if let Some(abbrevs) = abbreviations.get("Corporation") {
///     println!("Corporation abbreviations: {:?}", abbrevs);
/// }
/// ```
pub fn get_case_name_abbreviations() -> &'static CaseNamePartAbbreviationMap {
  case_name_part_abbreviations::case_name_part_abbreviations()
}

/// Get the legal journals database.
///
/// Returns a map from journal abbreviation keys to lists of [`Journal`] structs.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_journals;
///
/// let journals = get_journals();
/// for (key, journal_list) in journals.entries().take(3) {
///     println!("Journal key: {}, count: {}", key, journal_list.len());
/// }
/// ```
pub fn get_journals() -> &'static JournalsMap {
  journals::journals()
}

/// Get the laws database for statute citations.
///
/// Returns a map from law abbreviation keys to lists of [`Law`] structs.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_laws;
///
/// let laws = get_laws();
/// for (key, law_list) in laws.entries().take(3) {
///     println!("Law key: {}, count: {}", key, law_list.len());
/// }
/// ```
pub fn get_laws() -> &'static LawsMap {
  laws::laws()
}

/// Get processed regex variables for citation pattern matching.
///
/// Returns a map from variable names to [`RegexTemplate`] values that can be used
/// for parsing legal citations. These templates support variable substitution.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_regex_variables;
///
/// let regex_vars = get_regex_variables();
/// if let Some(template) = regex_vars.get("full_cite") {
///     println!("Full cite pattern: {}", template.value());
/// }
/// ```
pub fn get_regex_variables() -> HashMap<String, RegexTemplate> {
  regexes::regexes()
}

// Convenience functions that mirror Python's pre-computed data structures

/// Get variations mapping that maps variation names to canonical reporter abbreviations.
///
/// This provides a direct lookup from common variations and alternate names
/// to their canonical reporter abbreviations.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_variations_only;
///
/// let variations = get_variations_only();
///
/// // Look up variations for a reporter name
/// if let Some(canonical_forms) = variations.get("Atlantic Reporter") {
///     println!("Atlantic Reporter canonical forms: {:?}", canonical_forms);
///     // Might print: ["A.", "A.2d", "A.3d"]
/// }
///
/// // Find what "F.3d" maps to
/// if let Some(forms) = variations.get("F.3d") {
///     println!("F.3d canonical forms: {:?}", forms);
/// }
/// ```
pub fn get_variations_only() -> &'static HashMap<String, Vec<String>> {
  utils::get_variations_only()
}

/// Get editions mapping that maps edition keys to their root reporter name.
///
/// This provides a reverse lookup from edition abbreviations to the reporter
/// that contains them.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_editions;
///
/// let editions = get_editions();
///
/// // Look up which reporter contains "F.3d"
/// if let Some(reporter_name) = editions.get("F.3d") {
///     println!("F.3d belongs to reporter: {}", reporter_name);
/// }
///
/// // Check multiple editions
/// for edition in ["A.2d", "F.3d", "S.Ct."] {
///     if let Some(reporter) = editions.get(edition) {
///         println!("{} -> {}", edition, reporter);
///     }
/// }
/// ```
pub fn get_editions() -> &'static HashMap<String, String> {
  utils::get_editions_mapping()
}

/// Get names to editions mapping that maps reporter names to their edition abbreviations.
///
/// This provides a lookup from full reporter names to all their edition abbreviations.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_names_to_editions;
///
/// let names_to_editions = get_names_to_editions();
///
/// // Look up all editions for Atlantic Reporter
/// if let Some(editions) = names_to_editions.get("Atlantic Reporter") {
///     println!("Atlantic Reporter editions: {:?}", editions);
///     // Might print: ["A.", "A.2d", "A.3d"]
/// }
///
/// // Browse first few reporters
/// for (name, editions) in names_to_editions.iter().take(3) {
///     println!("{}: {:?}", name, editions);
/// }
/// ```
pub fn get_names_to_editions() -> &'static HashMap<String, Vec<String>> {
  utils::get_names_to_editions()
}

/// Get special formats mapping for non-standard citation formats.
///
/// This provides specialized formatting rules for reporters that don't follow
/// standard citation patterns.
///
/// # Examples
///
/// ```rust
/// use reporters_db::get_special_formats;
///
/// let formats = get_special_formats();
///
/// // Check if there are special formatting rules
/// for (edition, format) in formats.iter().take(5) {
///     println!("Special format for {}: {}", edition, format);
/// }
/// ```
pub fn get_special_formats() -> &'static HashMap<String, String> {
  utils::get_formats_mapping()
}

#[cfg(test)]
mod tests {
  use super::{
    get_case_name_abbreviations, get_editions, get_journals, get_laws, get_names_to_editions,
    get_regex_variables, get_reporters, get_special_formats, get_state_abbreviations,
    get_variations_only,
  };
  use std::collections::HashSet;

  #[test]
  fn test_basic_api_functions() {
    // Test that all main API functions return non-empty data
    assert!(
      !get_reporters().is_empty(),
      "Reporters database should not be empty"
    );
    assert!(
      !get_state_abbreviations().is_empty(),
      "State abbreviations should not be empty"
    );
    assert!(
      !get_case_name_abbreviations().is_empty(),
      "Case name abbreviations should not be empty"
    );
    assert!(!get_journals().is_empty(), "Journals should not be empty");
    assert!(!get_laws().is_empty(), "Laws should not be empty");
    assert!(
      !get_regex_variables().is_empty(),
      "Regex variables should not be empty"
    );
  }

  #[test]
  fn test_convenience_functions() {
    // Test convenience functions return expected data
    let variations = get_variations_only();
    let editions = get_editions();
    let names_to_editions = get_names_to_editions();
    let _formats = get_special_formats();

    assert!(!variations.is_empty(), "Variations should not be empty");
    assert!(!editions.is_empty(), "Editions should not be empty");
    assert!(
      !names_to_editions.is_empty(),
      "Names to editions should not be empty"
    );
    // Formats might be empty as noted in the implementation
  }

  #[test]
  fn test_variations_consistency() {
    let variations = get_variations_only();
    let reporters = get_reporters();

    // Test that all variations point to valid reporters
    let mut missing_count = 0;
    for (variation, canonical_list) in variations {
      assert!(
        !canonical_list.is_empty(),
        "Variation '{}' should have at least one canonical form",
        variation
      );
      for canonical in canonical_list {
        if !reporters.contains_key(canonical) {
          missing_count += 1;
          eprintln!(
            "Warning: Canonical reporter '{}' for variation '{}' not found in reporters",
            canonical, variation
          );
        }
      }
    }

    // Allow missing reporters but log them - this is expected due to historical data
    let total_canonicals: usize = variations.values().map(|v| v.len()).sum();
    let missing_percentage = (missing_count as f32 / total_canonicals as f32) * 100.0;
    println!(
      "Missing canonical reporters: {:.1}% ({}/{})",
      missing_percentage, missing_count, total_canonicals
    );

    // Ensure we have at least some valid mappings
    assert!(
      missing_percentage < 50.0,
      "Too many missing canonical reporters: {:.1}% ({}/{})",
      missing_percentage,
      missing_count,
      total_canonicals
    );
  }

  #[test]
  fn test_editions_consistency() {
    let editions = get_editions();
    let reporters = get_reporters();

    // Test that all edition mappings point to valid reporters
    for (edition, reporter_key) in editions {
      assert!(
        reporters.contains_key(reporter_key),
        "Reporter '{}' for edition '{}' should exist",
        reporter_key,
        edition
      );
    }
  }

  #[test]
  fn test_names_to_editions_consistency() {
    let names_to_editions = get_names_to_editions();
    let reporters = get_reporters();

    // Test that all names correspond to valid reporters
    for (name, edition_list) in names_to_editions {
      assert!(
        !edition_list.is_empty(),
        "Name '{}' should have at least one edition",
        name
      );

      // Find a reporter with this name
      let found_reporter = reporters
        .values()
        .flat_map(|reporter_list| reporter_list.iter())
        .any(|reporter| &reporter.name == name);

      assert!(
        found_reporter,
        "Name '{}' should correspond to an actual reporter",
        name
      );
    }
  }

  #[test]
  fn test_cite_types_validity() {
    let reporters = get_reporters();
    let valid_cite_types: HashSet<&str> = [
      "federal",
      "neutral",
      "scotusearly",
      "specialty",
      "specialtywest",
      "specialtylexis",
      "state",
      "stateregional",
    ]
    .iter()
    .cloned()
    .collect();

    for (reporter_key, reporter_list) in reporters {
      for reporter in reporter_list.iter() {
        let cite_type_str = format!("{:?}", reporter.cite_type).to_lowercase();
        assert!(
          valid_cite_types.contains(cite_type_str.as_str()),
          "Reporter '{}' has invalid cite_type: '{:?}'",
          reporter_key,
          reporter.cite_type
        );
      }
    }
  }

  #[test]
  fn test_date_format_consistency() {
    let reporters = get_reporters();

    for (reporter_key, reporter_list) in reporters {
      for reporter in reporter_list.iter() {
        for (edition_key, edition) in reporter.editions {
          if let Some(start) = edition.start {
            assert!(
              start.len() >= 10,
              "Start date for {}:{} should be at least 10 characters",
              reporter_key,
              edition_key
            );
            assert!(
              start.contains('-'),
              "Start date for {}:{} should contain hyphens",
              reporter_key,
              edition_key
            );
          }
          if let Some(end) = edition.end {
            assert!(
              end.len() >= 10,
              "End date for {}:{} should be at least 10 characters",
              reporter_key,
              edition_key
            );
            assert!(
              end.contains('-'),
              "End date for {}:{} should contain hyphens",
              reporter_key,
              edition_key
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_regex_variables_resolution() {
    let regex_vars = get_regex_variables();

    // Test that regex variables don't contain unresolved references
    for (key, template) in &regex_vars {
      let value = template.value();
      // Simple check that we don't have obvious unresolved references
      assert!(
        !value.contains("$UNRESOLVED"),
        "Variable '{}' contains unresolved reference: '{}'",
        key,
        value
      );
    }
  }
}
