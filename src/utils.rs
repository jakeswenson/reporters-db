use crate::regexes::{RawRegexMap, RegexOrNested, RegexTemplate};
use crate::reporters::reporters;
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

fn flatten(map: RawRegexMap) -> impl Iterator<Item = (String, RegexTemplate)> {
  map
    .into_iter()
    .filter(|(k, _)| !k.ends_with('#'))
    .flat_map(|(name, value)| match value {
      RegexOrNested::Regex(template) => vec![(name, template)],
      RegexOrNested::Nested(map) => flatten(map)
        .map(|(k, v)| {
          if k.is_empty() {
            (name.clone(), v)
          } else {
            (format!("{}_{}", name, k), v)
          }
        })
        .collect(),
    })
}

/// Process contents of variables.json, in preparation for passing to recursive_substitute:
/// - Strip keys ending in '#', which are treated as comments
/// - Flatten nested dicts, so {"page": {"": "A", "foo": "B"}} becomes {"page": "A", "page_foo": "B"}
/// - Add optional variants for each key, so {"page": "\\d+"} becomes {"page_optional": "(?:\\d+ ?)?"}
/// - Resolve nested references
pub fn process_variables(raw_regexes: RawRegexMap) -> HashMap<String, RegexTemplate> {
  let variables: HashMap<_, _> = flatten(raw_regexes)
    .flat_map(|(name, value)| {
      vec![
        (name.clone(), value.clone()),
        (
          format!("{}_optional", name),
          RegexTemplate::of(format!("(?:{} ?)?", value.value())),
        ),
      ]
    })
    .collect();

  variables
    .clone()
    .into_iter()
    .map(|(k, v)| (k, recursive_substitute(v, &variables)))
    .collect()
}

/// Recursively substitute values in `template` from `variables`. For example:
///         >>> recursive_substitute("$a $b $c", {'a': '$b', 'b': '$c', 'c': 'foo'})
///         "foo foo foo"
///     Infinite loops will raise a ValueError after max_depth loops.
pub fn recursive_substitute(
  template: RegexTemplate,
  map: &HashMap<String, RegexTemplate>,
) -> RegexTemplate {
  let mut new_value = template.clone();
  for _ in 0..100 {
    new_value = new_value.resolve(map);
    if new_value == template {
      break;
    }
  }

  new_value
}

/// Builds a dictionary of variations to canonical reporters.
///
/// The dictionary takes the form of:
///     {
///      "A. 2d": ["A.2d"],
///      ...
///      "P.R.": ["Pen. & W.", "P.R.R.", "P."],
///     }
///
/// In other words, it's a dictionary that maps each variation to a list of
/// reporters that it could be possibly referring to.
pub fn variations_only() -> HashMap<String, Vec<String>> {
  let mut variations_out = HashMap::new();
  let reporters_data = reporters();

  for (_reporter_key, reporter_list) in reporters_data {
    for reporter in *reporter_list {
      if let Some(variations) = &reporter.variations {
        for (variation_key, variation_value) in *variations {
          variations_out
            .entry(variation_key.to_string())
            .or_insert_with(Vec::new)
            .push(variation_value.to_string());
        }
      }
    }
  }

  variations_out
}

/// Builds a dictionary mapping edition keys to their root name.
///
/// The dictionary takes the form of:
///     {
///      "A.":   "A.",
///      "A.2d": "A.",
///      "A.3d": "A.",
///      "A.D.": "A.D.",
///      ...
///     }
///
/// In other words, this lets you go from an edition match to its parent key.
pub fn editions_mapping() -> HashMap<String, String> {
  let mut editions_out = HashMap::new();
  let reporters_data = reporters();

  for (reporter_key, reporter_list) in reporters_data {
    for reporter in *reporter_list {
      for (edition_key, _edition_value) in reporter.editions {
        editions_out.insert(edition_key.to_string(), reporter_key.to_string());
      }
    }
  }

  editions_out
}

/// Build a dict mapping names to their variations
///
/// Something like:
///
/// ```
/// # use reporters_db::get_names_to_editions;
/// let names_to_editions = get_names_to_editions();
/// // {"Atlantic Reporter": ["A.", "A.2d"]}
/// ```
///
/// Note that the abbreviations are sorted by start date.
pub fn names_to_editions() -> HashMap<String, Vec<String>> {
  let mut names = HashMap::new();
  let reporters_data = reporters();

  for (_reporter_key, reporter_list) in reporters_data {
    for reporter in *reporter_list {
      let mut abbrevs: Vec<String> = reporter.editions.keys().map(|k| k.to_string()).collect();

      // Sort abbreviations by start date of the edition
      abbrevs.sort_by(|a, b| {
        let start_a = reporter
          .editions
          .get(a)
          .and_then(|e| e.start)
          .unwrap_or("1750-01-01T00:00:00");
        let start_b = reporter
          .editions
          .get(b)
          .and_then(|e| e.start)
          .unwrap_or("1750-01-01T00:00:00");
        format!("{}{}", start_a, a).cmp(&format!("{}{}", start_b, b))
      });

      names.insert(reporter.name.to_string(), abbrevs);
    }
  }

  names
}

/// Builds a dictionary mapping edition keys to their cite_format if any.
///
/// The dictionary takes the form of:
///     {
///         'T.C. Summary Opinion': '{reporter} {volume}-{page}',
///         'T.C. Memo.': '{reporter} {volume}-{page}'
///         ...
///     }
pub fn formats_mapping() -> HashMap<String, String> {
  let formats_out = HashMap::new();
  let reporters_data = reporters();

  for (_reporter_key, reporter_list) in reporters_data {
    for reporter in *reporter_list {
      // Note: The current Reporter struct doesn't have a cite_format field
      // This would need to be added if cite_format support is needed
      for (_edition_key, _edition_value) in reporter.editions {
        // Placeholder - would extract cite_format if it existed
        // formats_out.insert(edition_key.clone(), reporter.cite_format.clone());
      }
    }
  }

  formats_out
}

/// Insert edition_name in place of $edition.
pub fn substitute_edition(
  regex: &str,
  edition_name: &str,
) -> String {
  let re = Regex::new(r"\$edition|\$\{edition\}").unwrap();
  re.replace_all(regex, regex::escape(edition_name))
    .to_string()
}

/// Insert edition strings for the given edition into a regex with an $edition placeholder.
///
/// Example:
/// substitute_editions(r"\d+ $edition \d+", "Foo.", {"Foo. Var.": "Foo."})
/// Returns: vec!["\\d+ (?:Foo\\.|Foo\\. Var\\.) \\d+"]
pub fn substitute_editions(
  regex: &str,
  edition_name: &str,
  variations: &HashMap<String, String>,
) -> Vec<String> {
  if !regex.contains("$edition") && !regex.contains("${edition}") {
    return vec![regex.to_string()];
  }

  let mut edition_strings = vec![edition_name.to_string()];
  for (k, v) in variations {
    if v == edition_name {
      edition_strings.push(k.clone());
    }
  }

  edition_strings
    .into_iter()
    .map(|e| substitute_edition(regex, &e))
    .collect()
}

// Lazy-initialized convenience data structures
static VARIATIONS_ONLY_DATA: LazyLock<HashMap<String, Vec<String>>> =
  LazyLock::new(variations_only);
static EDITIONS_MAPPING_DATA: LazyLock<HashMap<String, String>> = LazyLock::new(editions_mapping);
static NAMES_TO_EDITIONS_DATA: LazyLock<HashMap<String, Vec<String>>> =
  LazyLock::new(names_to_editions);
static FORMATS_MAPPING_DATA: LazyLock<HashMap<String, String>> = LazyLock::new(formats_mapping);

/// Get the pre-computed variations mapping
pub fn get_variations_only() -> &'static HashMap<String, Vec<String>> {
  &VARIATIONS_ONLY_DATA
}

/// Get the pre-computed editions mapping
pub fn get_editions_mapping() -> &'static HashMap<String, String> {
  &EDITIONS_MAPPING_DATA
}

/// Get the pre-computed names to editions mapping
pub fn get_names_to_editions() -> &'static HashMap<String, Vec<String>> {
  &NAMES_TO_EDITIONS_DATA
}

/// Get the pre-computed formats mapping
pub fn get_formats_mapping() -> &'static HashMap<String, String> {
  &FORMATS_MAPPING_DATA
}
