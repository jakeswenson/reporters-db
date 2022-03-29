use crate::regexes::{RawRegexMap, RegexOrNested, RegexTemplate};
use std::collections::HashMap;

fn flatten(map: RawRegexMap) -> impl Iterator<Item = (String, RegexTemplate)> {
    map.into_iter()
        .filter(|(k, _)| !k.ends_with('#'))
        .flat_map(|(name, value)| match value {
            RegexOrNested::Regex(template) => vec![(name, template)],
            RegexOrNested::Nested(map) => flatten(map)
                .into_iter()
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

/**!
Process contents of variables.json, in preparation for passing to recursive_substitute:
- Strip keys ending in '#', which are treated as comments
- Flatten nested dicts, so {"page": {"": "A", "foo": "B"}} becomes {"page": "A", "page_foo": "B"}
- Add optional variants for each key, so {"page": "\\d+"} becomes {"page_optional": "(?:\\d+ ?)?"}
- Resolve nested references
 */
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

/**!
Recursively substitute values in `template` from `variables`. For example:
        >>> recursive_substitute("$a $b $c", {'a': '$b', 'b': '$c', 'c': 'foo'})
        "foo foo foo"
    Infinite loops will raise a ValueError after max_depth loops.
 */
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
