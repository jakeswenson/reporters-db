/// Shared types for JSON parsing in build.rs
///
/// This module contains data structures with owned types (String, Vec<String>, etc.)
/// used for parsing JSON at compile-time in build.rs. The generated code creates
/// optimized versions with static references (&'static str) for zero-copy runtime access.
use serde::Deserialize;
use std::collections::HashMap;

/// A legal reporter with its editions and metadata
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Reporter {
  pub cite_type: CiteType,
  pub editions: HashMap<String, Edition>,
  pub mlz_jurisdiction: Vec<String>,
  pub name: String,
  pub variations: HashMap<String, String>,
  #[serde(default)]
  pub href: Option<String>,
}

/// Classification of legal citation types for reporters.
///
/// This enum categorizes different types of legal publications based on their
/// jurisdiction and scope. Each type has different citation formats and rules.
///
/// # Examples
///
/// ```rust
/// use reporters_db::{get_reporters, CiteType};
///
/// let reporters = get_reporters();
/// if let Some(reporter_list) = reporters.get("F.3d") {
///     for reporter in reporter_list.iter() {
///         match reporter.cite_type {
///             CiteType::Federal => println!("Federal court reporter"),
///             CiteType::State => println!("State court reporter"),
///             _ => println!("Other type: {:?}", reporter.cite_type),
///         }
///     }
/// }
/// ```
#[derive(Deserialize, Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CiteType {
  /// Multi-state regional reporters (e.g., A.2d, P.3d).
  StateRegional,
  /// Individual state court reporters.
  State,
  /// Federal court reporters (e.g., F.3d, U.S.).
  Federal,
  /// Specialty or topical reporters.
  Specialty,
  /// Neutral citations (jurisdiction-independent).
  Neutral,
  /// Specialty reporters published by LexisNexis.
  SpecialtyLexis,
  /// Early Supreme Court reporters (pre-1875).
  ScotusEarly,
  /// Specialty reporters published by West Publishing.
  SpecialtyWest,
}

/// An edition of a legal reporter with date ranges and regex patterns
#[derive(Deserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edition {
  pub end: Option<String>,
  pub start: Option<String>,
  #[serde(default)]
  pub regexes: Option<Vec<String>>,
}

/// A legal journal entry
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Journal {
  pub cite_type: String,
  pub name: String,
  #[serde(default)]
  pub start: Option<String>,
  #[serde(default)]
  pub end: Option<String>,
  #[serde(default)]
  pub examples: Vec<String>,
  #[serde(default)]
  pub regexes: Vec<String>,
  #[serde(default)]
  pub notes: Option<String>,
  #[serde(default)]
  pub href: Option<String>,
  #[serde(default)]
  #[allow(dead_code)]
  pub variations: Vec<String>,
}

/// A legal law/statute reference
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Law {
  pub cite_type: String,
  pub name: String,
  pub jurisdiction: String,
  #[serde(default)]
  pub start: Option<String>,
  #[serde(default)]
  pub end: Option<String>,
  #[serde(default)]
  pub examples: Vec<String>,
  #[serde(default)]
  pub regexes: Vec<String>,
  #[serde(default)]
  pub notes: Option<String>,
  #[serde(default)]
  pub href: Option<String>,
  #[serde(default)]
  #[allow(dead_code)]
  pub variations: Vec<String>,
}
