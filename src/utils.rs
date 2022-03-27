

use std::collections::HashMap;
use regex::{Captures, Regex};
use crate::regexes::{RawRegexMap, RegexOrNested, RegexTemplate};

fn flatten(map: RawRegexMap) -> HashMap<String, RegexTemplate> {
  map.into_iter()
    .filter(|(k, _)| !k.ends_with('#'))
    .flat_map(|(name, value)| match value {
      RegexOrNested::Regex(template) => vec![(name, template)],
      RegexOrNested::Nested(map) =>
        flatten(map)
          .into_iter()
          .map(|(k, v)| (format!("{}_{}", name, k), v))
          .collect()
    })
    .flat_map(|(name, value)| vec![
      (name.clone(), value.clone()),
      (format!("{}_optional", name), value)
    ])
    .collect()
}

/**!
Process contents of variables.json, in preparation for passing to recursive_substitute:
- Strip keys ending in '#', which are treated as comments
- Flatten nested dicts, so {"page": {"": "A", "foo": "B"}} becomes {"page": "A", "page_foo": "B"}
- Add optional variants for each key, so {"page": "\\d+"} becomes {"page_optional": "(?:\\d+ ?)?"}
- Resolve nested references
 */
pub(crate) fn process_variables(map: RawRegexMap) -> HashMap<String, RegexTemplate> {
  let variables = flatten(map);

  variables.clone().into_iter().map(|(k, mut v)| {
    recursive_substitute(&mut v, &variables);
    (k, v)
  }).collect()
}

/**!
Recursively substitute values in `template` from `variables`. For example:
        >>> recursive_substitute("$a $b $c", {'a': '$b', 'b': '$c', 'c': 'foo'})
        "foo foo foo"
    Infinite loops will raise a ValueError after max_depth loops.
 */
pub(crate) fn recursive_substitute(template: &mut RegexTemplate, map: &HashMap<String, RegexTemplate>) {
  let result = Regex::new(r"$(\w+)").unwrap();

  for _ in 0..100 {
    let new_value = result.replace_all(&template.0, |caps: &Captures| {
      &map.get(&caps[1]).unwrap().0
    });

    if new_value != template.0 {
      break;
    }

    template.0 = new_value.to_string();
  }
}
