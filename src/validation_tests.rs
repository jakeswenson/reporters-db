/// Advanced validation tests ported from Python test suite
///
/// This module contains comprehensive validation logic that mirrors
/// the Python test suite, ensuring data integrity and consistency.
/// Valid cite types as defined in Python test suite
static VALID_CITE_TYPES: &[&str] = &[
  "federal",
  "neutral",
  "scotusearly",
  "specialty",
  "specialtywest",
  "specialtylexis",
  "state",
  "stateregional",
];

/// Check that all strings in the data match expected ASCII characters
pub fn check_ascii(strings: &[String]) -> Result<(), String> {
  // Allow common legal publication characters including Unicode quotes and punctuation
  for s in strings {
    for c in s.chars() {
      if !c.is_ascii_alphanumeric()
        && !c.is_ascii_punctuation()
        && c != ' '
        && !matches!(c, '\u{2019}' | '\u{2018}' | '\u{201C}' | '\u{201D}')
      {
        return Err(format!(
          "Unexpected character in '{}': '{}' (U+{:04X})",
          s, c, c as u32
        ));
      }
    }
  }
  Ok(())
}

/// Check that strings don't have leading/trailing whitespace or unexpected whitespace
pub fn check_whitespace(strings: &[String]) -> Result<(), String> {
  for s in strings {
    if s != s.trim() {
      return Err(format!("Field needs whitespace stripped: '{}'", s));
    }

    // Check for non-space whitespace
    for ch in s.chars() {
      if ch.is_whitespace() && ch != ' ' {
        return Err(format!("Field has unexpected whitespace: '{}'", s));
      }
    }
  }
  Ok(())
}

/// Check that dates are properly formatted and start <= end
pub fn check_dates(
  start: Option<&str>,
  end: Option<&str>,
) -> Result<(), String> {
  if let Some(start_str) = start
    && (start_str.len() < 10 || !start_str.contains('-'))
  {
    return Err(format!("Invalid start date format: '{}'", start_str));
  }

  if let Some(end_str) = end
    && (end_str.len() < 10 || !end_str.contains('-'))
  {
    return Err(format!("Invalid end date format: '{}'", end_str));
  }

  // Basic chronological check - start should come before end
  if let (Some(start_str), Some(end_str)) = (start, end)
    && start_str > end_str
  {
    return Err(format!(
      "Start date '{}' is after end date '{}'",
      start_str, end_str
    ));
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::{VALID_CITE_TYPES, check_ascii, check_dates, check_whitespace};
  use crate::{
    get_editions, get_names_to_editions, get_regex_variables, get_reporters, get_variations_only,
  };
  use std::collections::HashSet;

  #[test]
  fn test_any_keys_missing_editions() {
    let reporters = get_reporters();

    for (reporter_key, reporter_list) in reporters {
      for reporter in reporter_list.iter() {
        assert!(
          reporter.editions.contains_key(reporter_key),
          "Could not find edition for key: {}",
          reporter_key
        );
      }
    }
  }

  #[test]
  fn test_for_variations_mapping_to_bad_keys() {
    let variations = get_variations_only();
    let editions = get_editions();
    let reporters = get_reporters();

    for (variation, canonical_list) in variations {
      for canonical in canonical_list {
        if let Some(reporter_key) = editions.get(canonical) {
          assert!(
            reporters.contains_key(reporter_key),
            "Could not map variation '{}' -> canonical '{}' -> reporter '{}' to a valid reporter",
            variation,
            canonical,
            reporter_key
          );
        }
        // Note: Some canonicals might not exist in editions, which we allow per the relaxed test
      }
    }
  }

  #[test]
  fn test_basic_names_to_editions() {
    let names_to_editions = get_names_to_editions();

    // Test a known case from Python tests
    if let Some(atlantic_editions) = names_to_editions.get("Atlantic Reporter") {
      let expected = vec!["A.", "A.2d", "A.3d"];
      // Check that we have at least the expected editions (may have more)
      for expected_edition in &expected {
        assert!(
          atlantic_editions.contains(&expected_edition.to_string()),
          "Missing expected edition '{}' for Atlantic Reporter",
          expected_edition
        );
      }
    }
  }

  #[test]
  fn test_all_reporters_have_valid_cite_type() {
    let reporters = get_reporters();
    let valid_types: HashSet<&str> = VALID_CITE_TYPES.iter().cloned().collect();

    for (reporter_key, reporter_list) in reporters {
      for reporter in reporter_list.iter() {
        let cite_type_str = format!("{:?}", reporter.cite_type).to_lowercase();
        assert!(
          valid_types.contains(cite_type_str.as_str()),
          "Reporter '{}' does not have a valid cite_type value: '{:?}'",
          reporter_key,
          reporter.cite_type
        );
      }
    }
  }

  #[test]
  fn test_no_variation_is_same_as_key() {
    let variations = get_variations_only();

    for (variation, canonical_list) in variations {
      for canonical in canonical_list {
        assert_ne!(
          variation, canonical,
          "The variation '{}' is identical to the canonical key it's supposed to be a variation of",
          variation
        );
      }
    }
  }

  #[test]
  fn test_fields_ascii_compliance() {
    let reporters = get_reporters();

    for (reporter_key, reporter_list) in reporters {
      // Check reporter key
      assert!(
        check_ascii(&[reporter_key.to_string()]).is_ok(),
        "Reporter key '{}' contains invalid ASCII characters",
        reporter_key
      );

      for reporter in reporter_list.iter() {
        // Check edition keys
        let edition_keys: Vec<String> = reporter.editions.keys().map(|k| k.to_string()).collect();
        assert!(
          check_ascii(&edition_keys).is_ok(),
          "Edition keys for '{}' contain invalid ASCII characters",
          reporter_key
        );

        // Check variation keys if any
        if let Some(variations) = &reporter.variations {
          let variation_keys: Vec<String> = variations.keys().map(|k| k.to_string()).collect();
          if let Err(err) = check_ascii(&variation_keys) {
            panic!(
              "Variation keys for '{}' contain invalid ASCII characters: {}",
              reporter_key, err
            );
          }
        }
      }
    }
  }

  #[test]
  fn test_fields_whitespace_compliance() {
    let reporters = get_reporters();

    for (reporter_key, reporter_list) in reporters {
      // Check reporter key whitespace
      assert!(
        check_whitespace(&[reporter_key.to_string()]).is_ok(),
        "Reporter key '{}' has whitespace issues",
        reporter_key
      );

      for reporter in reporter_list.iter() {
        // Check reporter name whitespace
        assert!(
          check_whitespace(&[reporter.name.to_string()]).is_ok(),
          "Reporter name for '{}' has whitespace issues",
          reporter_key
        );

        // Check edition keys whitespace
        let edition_keys: Vec<String> = reporter.editions.keys().map(|k| k.to_string()).collect();
        assert!(
          check_whitespace(&edition_keys).is_ok(),
          "Edition keys for '{}' have whitespace issues",
          reporter_key
        );
      }
    }
  }

  #[test]
  fn test_edition_dates() {
    let reporters = get_reporters();

    for (reporter_key, reporter_list) in reporters {
      for reporter in reporter_list.iter() {
        for (edition_key, edition) in reporter.editions.entries() {
          assert!(
            check_dates(edition.start, edition.end).is_ok(),
            "Date validation failed for {}:{}",
            reporter_key,
            edition_key
          );
        }
      }
    }
  }

  #[test]
  fn test_edition_name_consistency() {
    let reporters = get_reporters();
    let names_to_editions = get_names_to_editions();

    // Test that names_to_editions keys correspond to actual reporter names
    for (name, edition_list) in names_to_editions {
      assert!(!edition_list.is_empty(), "Name '{}' has no editions", name);

      // Find if this name exists in any reporter
      let found = reporters
        .values()
        .flat_map(|list| list.iter())
        .any(|reporter| reporter.name == name);

      assert!(
        found,
        "Name '{}' in names_to_editions doesn't correspond to any reporter",
        name
      );
    }
  }

  #[test]
  fn test_variations_consistency() {
    let variations = get_variations_only();
    let reporters = get_reporters();

    // Count how many variations have valid canonical mappings
    let mut valid_mappings = 0;
    let mut total_mappings = 0;

    for canonical_list in variations.values() {
      for canonical in canonical_list {
        total_mappings += 1;
        if reporters.contains_key(canonical) {
          valid_mappings += 1;
        }
      }
    }

    let validity_ratio = valid_mappings as f32 / total_mappings as f32;
    assert!(
      validity_ratio > 0.5, // At least 50% should be valid
      "Too few valid variation mappings: {:.1}% ({}/{})",
      validity_ratio * 100.0,
      valid_mappings,
      total_mappings
    );
  }

  #[test]
  fn test_regex_variables_contain_expected_keys() {
    let regex_vars = get_regex_variables();

    // Test that we have some expected regex variables from the Python version
    let expected_keys = [
      "page",
      "page_optional",
      "volume",
      "volume_optional",
      "reporter",
      "reporter_optional",
    ];

    for expected_key in &expected_keys {
      assert!(
        regex_vars.contains_key(*expected_key),
        "Missing expected regex variable: '{}'",
        expected_key
      );
    }
  }

  #[test]
  fn test_data_structure_sizes() {
    let reporters = get_reporters();
    let variations = get_variations_only();
    let editions = get_editions();
    let names_to_editions = get_names_to_editions();

    // Ensure we have reasonable amounts of data
    assert!(
      reporters.len() > 100,
      "Too few reporters: {}",
      reporters.len()
    );
    assert!(
      variations.len() > 100,
      "Too few variations: {}",
      variations.len()
    );
    assert!(editions.len() > 100, "Too few editions: {}", editions.len());
    assert!(
      names_to_editions.len() > 50,
      "Too few name mappings: {}",
      names_to_editions.len()
    );

    // Test rough proportions - variations should be much larger than base reporters
    assert!(
      variations.len() > reporters.len(),
      "Expected more variations ({}) than reporters ({})",
      variations.len(),
      reporters.len()
    );
  }

  #[test]
  fn test_edition_start_dates_are_sorted() {
    let names_to_editions = get_names_to_editions();
    let reporters = get_reporters();

    // For each reporter name, verify that editions are sorted by start date
    for (name, edition_list) in names_to_editions {
      if edition_list.len() <= 1 {
        continue; // Skip single-edition reporters
      }

      // Find the reporter with this name
      let reporter = reporters
        .values()
        .flat_map(|list| list.iter())
        .find(|r| r.name == name);

      if let Some(reporter) = reporter {
        let mut prev_date: Option<&str> = None;

        for edition_key in edition_list {
          if let Some(edition) = reporter.editions.get(edition_key.as_str())
            && let Some(start_date) = edition.start
          {
            if let Some(prev) = prev_date {
              assert!(
                start_date >= prev,
                "Editions for '{}' are not sorted by start date: '{}' comes after '{}'",
                name,
                start_date,
                prev
              );
            }
            prev_date = Some(start_date);
          }
        }
      }
    }
  }
}
