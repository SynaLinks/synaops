// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)

use std::collections::HashMap;

use regex::Regex;
use serde_json::{Map, Value};

use crate::nlp_utils::{
    add_suffix, is_plural, to_plural_without_numerical_suffix, to_singular_without_numerical_suffix,
};
use crate::regex_cache;

/// Add a prefix to the JSON object keys.
pub fn prefix_json(json: Value, prefix: &str) -> Value {
    let map = match json {
        Value::Object(m) => m,
        other => return other,
    };
    let mut result = Map::with_capacity(map.len());
    for (key, value) in map {
        let mut new_key = String::with_capacity(prefix.len() + 1 + key.len());
        new_key.push_str(prefix);
        new_key.push('_');
        new_key.push_str(&key);
        result.insert(new_key, value);
    }
    Value::Object(result)
}

/// Add a suffix to the JSON object keys.
pub fn suffix_json(json: Value, suffix: &str) -> Value {
    let map = match json {
        Value::Object(m) => m,
        other => return other,
    };
    let mut result = Map::with_capacity(map.len());
    for (key, value) in map {
        let mut new_key = String::with_capacity(key.len() + 1 + suffix.len());
        new_key.push_str(&key);
        new_key.push('_');
        new_key.push_str(suffix);
        result.insert(new_key, value);
    }
    Value::Object(result)
}

/// Concatenate two JSON objects into a single object.
///
/// Merges the properties of two JSON objects. If there are conflicting
/// property names, appends a numerical suffix to make them unique.
pub fn concatenate_json(json1: Value, json2: Value) -> Value {
    let map1 = match json1 {
        Value::Object(m) => m,
        _ => Map::new(),
    };
    let map2 = match json2 {
        Value::Object(m) => m,
        _ => Map::new(),
    };

    let mut result = Map::with_capacity(map1.len() + map2.len());

    let mut add_property = |key: String, value: Value| {
        if !result.contains_key(&key) {
            result.insert(key, value);
            return;
        }
        let mut suffix = 1usize;
        loop {
            let candidate = add_suffix(&key, suffix);
            if !result.contains_key(&candidate) {
                result.insert(candidate, value);
                return;
            }
            suffix += 1;
        }
    };

    for (key, value) in map1 {
        add_property(key, value);
    }
    for (key, value) in map2 {
        add_property(key, value);
    }

    Value::Object(result)
}

/// Factorize a JSON object by grouping similar properties into arrays.
///
/// Identifies similar properties based on their base names and creates
/// arrays for them.
pub fn factorize_json(json: Value) -> Value {
    let map = match json {
        Value::Object(m) => m,
        other => return other,
    };

    // Precompute base form and plural flag per key once (O(n) instead of O(n²)
    // recomputing inside a per-key filter), and count group sizes so sibling
    // lookup is O(1).
    let mut base_keys: Vec<String> = Vec::with_capacity(map.len());
    let mut plural_flags: Vec<bool> = Vec::with_capacity(map.len());
    let mut counts: HashMap<String, usize> = HashMap::with_capacity(map.len());
    for key in map.keys() {
        let base = to_singular_without_numerical_suffix(key);
        *counts.entry(base.clone()).or_insert(0) += 1;
        base_keys.push(base);
        plural_flags.push(is_plural(key));
    }

    let mut result: Map<String, Value> = Map::with_capacity(map.len());

    for (i, (prop_key, prop_value)) in map.into_iter().enumerate() {
        let base_key = &base_keys[i];
        let is_plural_key = plural_flags[i];
        let has_siblings = counts.get(base_key).copied().unwrap_or(0) > 1;

        if has_siblings && !is_plural_key {
            let plural_key = to_plural_without_numerical_suffix(base_key);
            let entry = result
                .entry(plural_key)
                .or_insert_with(|| Value::Array(Vec::new()));
            let arr = match entry {
                Value::Array(a) => a,
                _ => unreachable!(),
            };
            match prop_value {
                Value::Array(items) => arr.extend(items),
                other => arr.push(other),
            }
        } else if !is_plural_key {
            result.insert(base_key.clone(), prop_value);
        } else {
            result.insert(prop_key, prop_value);
        }
    }

    Value::Object(result)
}

/// Mask specific fields of a JSON object (remove matching properties).
///
/// `pattern` is an optional regex whose `.search()` (substring match) is tested
/// against each base property key. A key matching `mask` OR `pattern` is removed.
pub fn out_mask_json(
    json: &Value,
    mask: Option<&[&str]>,
    pattern: Option<&str>,
    recursive: bool,
) -> Result<Value, regex::Error> {
    let has_mask = matches!(mask, Some(m) if !m.is_empty());
    if !has_mask && pattern.is_none() {
        return Ok(json.clone());
    }

    let mask_vec: Vec<String> = mask
        .map(|m| {
            m.iter()
                .map(|k| to_singular_without_numerical_suffix(k))
                .collect()
        })
        .unwrap_or_default();

    let compiled = pattern.map(regex_cache::compile).transpose()?;

    Ok(out_mask_value(json, &mask_vec, compiled.as_ref(), recursive))
}

fn out_mask_value(
    value: &Value,
    mask: &[String],
    pattern: Option<&Regex>,
    recursive: bool,
) -> Value {
    match value {
        Value::Object(map) => {
            let mut result = Map::with_capacity(map.len());
            for (key, val) in map {
                let base_key = to_singular_without_numerical_suffix(key);
                if mask.contains(&base_key) {
                    continue;
                }
                if let Some(re) = pattern {
                    if re.find(&base_key).is_some() {
                        continue;
                    }
                }
                if recursive {
                    result.insert(key.clone(), out_mask_value(val, mask, pattern, true));
                } else {
                    result.insert(key.clone(), val.clone());
                }
            }
            Value::Object(result)
        }
        Value::Array(arr) if recursive => Value::Array(
            arr.iter()
                .map(|item| match item {
                    Value::Object(_) => out_mask_value(item, mask, pattern, true),
                    _ => item.clone(),
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}

/// Keep specific fields of a JSON object (remove all others).
///
/// `pattern` is an optional regex; a key is kept if its base form is in `mask`
/// OR matches `pattern`. With `recursive=true`, list-valued keys are always kept
/// and dict items inside lists are filtered in place.
pub fn in_mask_json(
    json: &Value,
    mask: Option<&[&str]>,
    pattern: Option<&str>,
    recursive: bool,
) -> Result<Value, regex::Error> {
    let has_mask = matches!(mask, Some(m) if !m.is_empty());
    if !has_mask && pattern.is_none() {
        return Ok(Value::Object(Map::new()));
    }

    let mask_vec: Vec<String> = mask
        .map(|m| {
            m.iter()
                .map(|k| to_singular_without_numerical_suffix(k))
                .collect()
        })
        .unwrap_or_default();

    let compiled = pattern.map(regex_cache::compile).transpose()?;

    Ok(in_mask_value(json, &mask_vec, compiled.as_ref(), recursive))
}

fn in_mask_value(
    value: &Value,
    mask: &[String],
    pattern: Option<&Regex>,
    recursive: bool,
) -> Value {
    match value {
        Value::Object(map) => {
            let mut result = Map::with_capacity(map.len());
            for (key, val) in map {
                let base_key = to_singular_without_numerical_suffix(key);
                let keep = mask.contains(&base_key)
                    || pattern.is_some_and(|re| re.find(&base_key).is_some());

                if recursive {
                    match val {
                        Value::Object(_) => {
                            if keep {
                                result.insert(
                                    key.clone(),
                                    in_mask_value(val, mask, pattern, true),
                                );
                            }
                        }
                        Value::Array(_) => {
                            result.insert(
                                key.clone(),
                                in_mask_value(val, mask, pattern, true),
                            );
                        }
                        _ => {
                            if keep {
                                result.insert(key.clone(), val.clone());
                            }
                        }
                    }
                } else if keep {
                    result.insert(key.clone(), val.clone());
                }
            }
            Value::Object(result)
        }
        Value::Array(arr) if recursive => Value::Array(
            arr.iter()
                .map(|item| match item {
                    Value::Object(_) => in_mask_value(item, mask, pattern, true),
                    _ => item.clone(),
                })
                .collect(),
        ),
        _ => value.clone(),
    }
}

/// Decompose a JSON object by expanding array properties into individual properties.
///
/// This is the inverse of `factorize_json`.
pub fn decompose_json(json: Value) -> Value {
    let map = match json {
        Value::Object(m) => m,
        other => return other,
    };

    let mut result = Map::with_capacity(map.len());

    for (prop_key, prop_value) in map {
        if is_plural(&prop_key) {
            if let Value::Array(items) = prop_value {
                let singular_key = to_singular_without_numerical_suffix(&prop_key);
                for (i, item) in items.into_iter().enumerate() {
                    if i == 0 {
                        result.insert(singular_key.clone(), item);
                    } else {
                        result.insert(add_suffix(&singular_key, i), item);
                    }
                }
            } else {
                result.insert(prop_key, prop_value);
            }
        } else {
            result.insert(prop_key, prop_value);
        }
    }

    Value::Object(result)
}
