// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)

use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};

use crate::regex_cache;

use crate::nlp_utils::{
    add_suffix, is_plural, to_plural_without_numerical_suffix, to_singular_without_numerical_suffix,
};

/// Extension trait providing `.insert`/`.remove` directly on `serde_json::Value`
/// (delegating to `as_object_mut`). Keeps the schema logic concise.
trait ValueExt {
    fn insert(&mut self, key: String, val: Value) -> Option<Value>;
    fn remove(&mut self, key: &str) -> Option<Value>;
}

impl ValueExt for Value {
    fn insert(&mut self, key: String, val: Value) -> Option<Value> {
        self.as_object_mut().and_then(|m| m.insert(key, val))
    }
    fn remove(&mut self, key: &str) -> Option<Value> {
        self.as_object_mut().and_then(|m| m.remove(key))
    }
}

/// Standardise the JSON schema for consistency.
pub fn standardize_schema(schema: Value) -> Value {
    schema
}

/// Move the value at `key` out of a root object Value, returning it without
/// cloning. Returns `None` if the root isn't an object or the key is absent.
fn take_key(schema: &mut Value, key: &str) -> Option<Value> {
    schema.as_object_mut()?.remove(key)
}

/// Same as `take_key` but specialised for extracting an owned `Map`.
fn take_object(schema: &mut Value, key: &str) -> Option<Map<String, Value>> {
    match take_key(schema, key)? {
        Value::Object(m) => Some(m),
        other => {
            // Not an object — put it back so we don't silently drop data.
            if let Some(obj) = schema.as_object_mut() {
                obj.insert(key.to_owned(), other);
            }
            None
        }
    }
}

fn default_title_from_key(key: &str) -> String {
    key.replace('_', " ")
        .split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            let c = chars.next().unwrap();
            let upper = c.to_uppercase().collect::<String>();
            let rest = chars.as_str();
            format!("{upper}{rest}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn titlecase(s: &str) -> String {
    default_title_from_key(s)
}

/// Add a prefix to the schema properties.
pub fn prefix_schema(mut schema: Value, prefix: &str) -> Value {
    let properties = take_object(&mut schema, "properties").unwrap_or_default();
    let n = properties.len();
    let mut new_properties = Map::with_capacity(n);
    let mut required: Vec<Value> = Vec::with_capacity(n);

    let tc = titlecase(prefix);
    for (prop_key, mut prop_value) in properties {
        let title = prop_value
            .get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| default_title_from_key(&prop_key));
        prop_value.insert("title".to_owned(), Value::String(format!("{tc} {title}")));
        let mut new_key = String::with_capacity(prefix.len() + 1 + prop_key.len());
        new_key.push_str(prefix);
        new_key.push('_');
        new_key.push_str(&prop_key);
        required.push(Value::String(new_key.clone()));
        new_properties.insert(new_key, prop_value);
    }

    schema.insert("properties".to_owned(), Value::Object(new_properties));
    schema.insert("required".to_owned(), Value::Array(required));
    schema
}

/// Add a suffix to the schema properties.
pub fn suffix_schema(mut schema: Value, suffix: &str) -> Value {
    let properties = take_object(&mut schema, "properties").unwrap_or_default();
    let n = properties.len();
    let mut new_properties = Map::with_capacity(n);
    let mut required: Vec<Value> = Vec::with_capacity(n);

    let tc = titlecase(suffix);
    for (prop_key, mut prop_value) in properties {
        let title = prop_value
            .get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned())
            .unwrap_or_else(|| default_title_from_key(&prop_key));
        prop_value.insert("title".to_owned(), Value::String(format!("{title} {tc}")));
        let mut new_key = String::with_capacity(prop_key.len() + 1 + suffix.len());
        new_key.push_str(&prop_key);
        new_key.push('_');
        new_key.push_str(suffix);
        required.push(Value::String(new_key.clone()));
        new_properties.insert(new_key, prop_value);
    }

    schema.insert("properties".to_owned(), Value::Object(new_properties));
    schema.insert("required".to_owned(), Value::Array(required));
    schema
}

/// Returns true if the schema is an object's schema.
pub fn is_object(schema: &Value) -> bool {
    schema
        .get("type")
        .and_then(|v| v.as_str())
        .is_some_and(|t| t == "object")
}

/// Returns true if the schema is an array's schema.
pub fn is_array(schema: &Value) -> bool {
    schema
        .get("type")
        .and_then(|v| v.as_str())
        .is_some_and(|t| t == "array")
}

/// Concatenate two JSON schemas into a single schema.
pub fn concatenate_schema(mut schema1: Value, mut schema2: Value) -> Value {
    let title = take_key(&mut schema1, "title").unwrap_or(Value::Null);

    let defs1 = take_object(&mut schema1, "$defs");
    let defs2 = take_object(&mut schema2, "$defs");
    let defs = match (defs1, defs2) {
        (Some(d1), None) => d1,
        (None, Some(d2)) => d2,
        (Some(mut d1), Some(d2)) => {
            d1.extend(d2);
            d1
        }
        (None, None) => Map::new(),
    };

    let schema1_properties = take_object(&mut schema1, "properties").unwrap_or_default();
    let schema2_properties = take_object(&mut schema2, "properties").unwrap_or_default();

    // Collect required names as HashSets for O(1) membership instead of Vec::contains.
    let required1: HashSet<String> = take_key(&mut schema1, "required")
        .and_then(|v| {
            if let Value::Array(arr) = v {
                Some(arr.into_iter().filter_map(|v| match v {
                    Value::String(s) => Some(s),
                    _ => None,
                }).collect())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let required2: HashSet<String> = take_key(&mut schema2, "required")
        .and_then(|v| {
            if let Value::Array(arr) = v {
                Some(arr.into_iter().filter_map(|v| match v {
                    Value::String(s) => Some(s),
                    _ => None,
                }).collect())
            } else {
                None
            }
        })
        .unwrap_or_default();

    let cap = schema1_properties.len() + schema2_properties.len();
    let mut out_props: Map<String, Value> = Map::with_capacity(cap);
    let mut out_required: Vec<Value> = Vec::with_capacity(cap);

    let has_req1 = !required1.is_empty();
    let has_req2 = !required2.is_empty();

    let mut add_property = |prop_key: String, mut prop_value: Value, in_req1: bool, in_req2: bool| {
        let (final_key, renamed) = if out_props.contains_key(&prop_key) {
            let mut suffix = 1usize;
            loop {
                let candidate = add_suffix(&prop_key, suffix);
                if !out_props.contains_key(&candidate) {
                    break (candidate, true);
                }
                suffix += 1;
            }
        } else {
            (prop_key, false)
        };

        if renamed {
            prop_value.insert(
                "title".to_owned(),
                Value::String(default_title_from_key(&final_key)),
            );
        }

        let should_add = match (has_req1, has_req2) {
            (true, false) => in_req1,
            (false, true) => in_req2,
            (true, true) => in_req1 || in_req2,
            (false, false) => false,
        };
        if should_add {
            out_required.push(Value::String(final_key.clone()));
        }
        out_props.insert(final_key, prop_value);
    };

    for (prop_key, prop_value) in schema1_properties {
        let in_req1 = required1.contains(&prop_key);
        let in_req2 = required2.contains(&prop_key);
        add_property(prop_key, prop_value, in_req1, in_req2);
    }
    for (prop_key, prop_value) in schema2_properties {
        let in_req1 = required1.contains(&prop_key);
        let in_req2 = required2.contains(&prop_key);
        add_property(prop_key, prop_value, in_req1, in_req2);
    }

    let mut result_schema = Value::Object(Map::with_capacity(6));
    result_schema.insert("additionalProperties".to_owned(), Value::Bool(false));
    if !defs.is_empty() {
        result_schema.insert("$defs".to_owned(), Value::Object(defs));
    }
    result_schema.insert("properties".to_owned(), Value::Object(out_props));
    result_schema.insert("required".to_owned(), Value::Array(out_required));
    result_schema.insert("title".to_owned(), title);
    result_schema.insert("type".to_owned(), Value::String("object".to_owned()));
    result_schema
}

/// Factorise a JSON schema by grouping similar properties into arrays.
pub fn factorize_schema(mut schema: Value) -> Value {
    let title = take_key(&mut schema, "title").unwrap_or(Value::Null);
    let defs = take_object(&mut schema, "$defs").unwrap_or_default();
    let schema_properties = take_object(&mut schema, "properties").unwrap_or_default();

    let n = schema_properties.len();
    let mut out_props: Map<String, Value> = Map::with_capacity(n);
    let mut out_required: Vec<Value> = Vec::with_capacity(n);

    // Precompute base form + plural flag once per key, and group indices by
    // base_key (O(n) instead of O(n²) rescans inside the per-key filter).
    let mut base_keys: Vec<String> = Vec::with_capacity(n);
    let mut plural_flags: Vec<bool> = Vec::with_capacity(n);
    let mut groups: HashMap<String, Vec<usize>> = HashMap::with_capacity(n);
    for (i, key) in schema_properties.keys().enumerate() {
        let base = to_singular_without_numerical_suffix(key);
        groups.entry(base.clone()).or_default().push(i);
        base_keys.push(base);
        plural_flags.push(is_plural(key));
    }
    // Move prop values out so the hot path can consume them directly instead
    // of cloning on every iteration. `take` replaces with `Value::Null`, which
    // is fine since we never re-read those slots.
    let mut prop_entries: Vec<(String, Value)> = schema_properties.into_iter().collect();

    // We read sibling values through shared refs into `prop_entries` while
    // mutating the current slot. Split via swap_remove pattern is awkward;
    // instead, we take each prop_value by `std::mem::take` when we need to
    // own it, and read siblings via indices into the still-present Values.
    for i in 0..prop_entries.len() {
        let base_key = &base_keys[i];
        let is_plural_key = plural_flags[i];
        let group = groups.get(base_key).unwrap();
        let has_siblings = group.len() > 1;

        if has_siblings && !is_plural_key {
            let plural_key = to_plural_without_numerical_suffix(base_key);
            if out_props.contains_key(&plural_key) {
                continue;
            }

            // Snapshot properties we still need from `prop_value` after it
            // gets moved / its `type` overwritten to "array" below.
            let prop_is_array = is_array(&prop_entries[i].1);
            let similar_all_array = group
                .iter()
                .filter(|&&j| j != i)
                .all(|&j| is_array(&prop_entries[j].1));
            let all_same_items = similar_all_array
                && group.iter().filter(|&&j| j != i).all(|&j| {
                    prop_entries[j].1.get("items") == prop_entries[i].1.get("items")
                });
            let orig_ref = prop_entries[i].1.get("$ref").cloned();
            let orig_type = prop_entries[i].1.get("type").cloned();

            let mut array_prop = std::mem::take(&mut prop_entries[i].1);
            array_prop.insert("title".to_owned(), Value::String(titlecase(&plural_key)));
            array_prop.insert("type".to_owned(), Value::String("array".to_owned()));

            if prop_is_array {
                // When all_same_items: array_prop already has the right `items` from the move.
                // When siblings aren't all arrays: keep prop_value's items as-is.
                if similar_all_array && !all_same_items {
                    // Collect anyOf from sibling items that differ from prop's items.
                    let prop_items_snapshot = array_prop.get("items").cloned();
                    if let Some(items) = array_prop.get_mut("items") {
                        items.remove("$ref");

                        let mut any_of: Vec<Value> = Vec::with_capacity(group.len());
                        for &j in group.iter().filter(|&&j| j != i) {
                            let sibling_items = prop_entries[j].1.get("items");
                            if sibling_items != prop_items_snapshot.as_ref() {
                                if let Some(s_items) = sibling_items {
                                    any_of.push(s_items.clone());
                                }
                            }
                        }
                        if let Some(items_val) = prop_items_snapshot {
                            any_of.push(items_val);
                        }
                        items.insert("anyOf".to_owned(), Value::Array(any_of));
                        array_prop.remove("description");
                    }
                }
            } else {
                // prop_value wasn't an array — synthesise an items object from
                // the *original* $ref/type snapshot (array_prop's type has
                // already been overwritten to "array" above).
                let mut items_obj = Map::with_capacity(1);
                if let Some(r) = orig_ref {
                    items_obj.insert("$ref".to_owned(), r);
                } else if let Some(t) = orig_type {
                    items_obj.insert("type".to_owned(), t);
                } else {
                    items_obj.insert(
                        "type".to_owned(),
                        Value::String("string".to_owned()),
                    );
                }
                array_prop.insert("items".to_owned(), Value::Object(items_obj));
            }

            out_required.push(Value::String(plural_key.clone()));
            out_props.insert(plural_key, array_prop);
        } else if !is_plural_key && !out_props.contains_key(base_key) {
            let prop_value = std::mem::take(&mut prop_entries[i].1);
            out_required.push(Value::String(base_key.clone()));
            out_props.insert(base_key.clone(), prop_value);
        }
    }

    let mut result_schema = Value::Object(Map::with_capacity(6));
    if !defs.is_empty() {
        result_schema.insert("$defs".to_owned(), Value::Object(defs));
    }
    result_schema.insert("additionalProperties".to_owned(), Value::Bool(false));
    result_schema.insert("properties".to_owned(), Value::Object(out_props));
    result_schema.insert("required".to_owned(), Value::Array(out_required));
    result_schema.insert("title".to_owned(), title);
    result_schema.insert("type".to_owned(), Value::String("object".to_owned()));
    result_schema
}

/// Resolve a key-path down through nested Maps to a mutable Value reference.
/// Returns `None` if any segment is missing or non-object along the way.
fn resolve_mut<'a>(root: &'a mut Value, path: &[String]) -> Option<&'a mut Value> {
    let mut cur = root;
    for key in path {
        cur = cur.as_object_mut()?.get_mut(key)?;
    }
    Some(cur)
}

/// Walk `value`, collecting every `#/$defs/<name>` target into `out`.
/// Sibling keys of `$ref` are still traversed (JSON Schema allows refs nested
/// in objects that hold their own subtrees, and the previous stringify-based
/// impl counted those too).
fn collect_referenced_defs(value: &Value, out: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                if k == "$ref" {
                    if let Value::String(s) = v {
                        if let Some(name) = s.strip_prefix("#/$defs/") {
                            out.insert(name.to_owned());
                        }
                    }
                } else {
                    collect_referenced_defs(v, out);
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                collect_referenced_defs(v, out);
            }
        }
        _ => {}
    }
}

fn prune_unreferenced_defs(schema: &mut Value) {
    if schema.get("$defs").and_then(|v| v.as_object()).is_none() {
        return;
    }
    let mut referenced: HashSet<String> = HashSet::new();
    collect_referenced_defs(schema, &mut referenced);
    if let Some(defs) = schema.get_mut("$defs").and_then(|v| v.as_object_mut()) {
        defs.retain(|k, _| referenced.contains(k));
    }
}

/// Mask specific fields of a JSON schema (remove matching properties).
///
/// A property key is removed if its base name is in `mask` OR matches `pattern`.
/// Iterative: maintains a stack of key-paths into the root schema.
pub fn out_mask_schema(
    mut schema: Value,
    mask: Option<&[&str]>,
    pattern: Option<&str>,
    recursive: bool,
) -> Result<Value, regex::Error> {
    let has_mask = matches!(mask, Some(m) if !m.is_empty());
    if !has_mask && pattern.is_none() {
        return Ok(schema);
    }

    let mask: Vec<String> = mask
        .map(|m| {
            m.iter()
                .map(|k| to_singular_without_numerical_suffix(k))
                .collect()
        })
        .unwrap_or_default();
    let compiled = pattern.map(regex_cache::compile).transpose()?;

    // Seed stack with the root and every $defs entry (Python does the same).
    let mut stack: Vec<Vec<String>> = vec![Vec::new()];
    if recursive {
        if let Some(defs) = schema.get("$defs").and_then(|v| v.as_object()) {
            for k in defs.keys() {
                stack.push(vec!["$defs".to_owned(), k.clone()]);
            }
        }
    }

    while let Some(path) = stack.pop() {
        let current = match resolve_mut(&mut schema, &path) {
            Some(v) => v,
            None => continue,
        };

        let mut keys_to_delete: Vec<String> = Vec::new();
        let mut children_to_push: Vec<Vec<String>> = Vec::new();

        if let Some(properties) = current.get("properties").and_then(|v| v.as_object()) {
            for (prop_key, prop_value) in properties {
                let base_key = to_singular_without_numerical_suffix(prop_key);
                if mask.contains(&base_key) {
                    keys_to_delete.push(prop_key.clone());
                } else if let Some(re) = compiled.as_ref() {
                    if re.find(&base_key).is_some() {
                        keys_to_delete.push(prop_key.clone());
                    }
                }

                if recursive {
                    if is_object(prop_value) {
                        let mut p = path.clone();
                        p.push("properties".to_owned());
                        p.push(prop_key.clone());
                        children_to_push.push(p);
                    } else if is_array(prop_value) {
                        let mut p = path.clone();
                        p.push("properties".to_owned());
                        p.push(prop_key.clone());
                        p.push("items".to_owned());
                        children_to_push.push(p);
                    }
                }
            }
        }

        if let Some(properties) = current
            .get_mut("properties")
            .and_then(|v| v.as_object_mut())
        {
            for key in &keys_to_delete {
                properties.remove(key);
            }
        }

        if let Some(Value::Array(req)) = current.get_mut("required") {
            req.retain(|v| {
                v.as_str()
                    .map(|s| !keys_to_delete.iter().any(|k| k == s))
                    .unwrap_or(true)
            });
        }

        stack.extend(children_to_push);
    }

    prune_unreferenced_defs(&mut schema);
    Ok(schema)
}

/// Keep specific fields of a JSON schema (remove all others).
///
/// A property key is kept if its base name is in `mask` OR matches `pattern`.
/// Iterative: maintains a stack of key-paths into the root schema.
pub fn in_mask_schema(
    mut schema: Value,
    mask: Option<&[&str]>,
    pattern: Option<&str>,
    recursive: bool,
) -> Result<Value, regex::Error> {
    let has_mask = matches!(mask, Some(m) if !m.is_empty());
    if !has_mask && pattern.is_none() {
        let title = take_key(&mut schema, "title").unwrap_or(Value::Null);
        let mut result = Value::Object(Map::with_capacity(4));
        result.insert("additionalProperties".to_owned(), Value::Bool(false));
        result.insert("properties".to_owned(), Value::Object(Map::new()));
        result.insert("title".to_owned(), title);
        result.insert("type".to_owned(), Value::String("object".to_owned()));
        return Ok(result);
    }

    let mask: Vec<String> = mask
        .map(|m| {
            m.iter()
                .map(|k| to_singular_without_numerical_suffix(k))
                .collect()
        })
        .unwrap_or_default();
    let compiled = pattern.map(regex_cache::compile).transpose()?;

    let mut stack: Vec<Vec<String>> = vec![Vec::new()];
    if recursive {
        if let Some(defs) = schema.get("$defs").and_then(|v| v.as_object()) {
            for k in defs.keys() {
                stack.push(vec!["$defs".to_owned(), k.clone()]);
            }
        }
    }

    while let Some(path) = stack.pop() {
        let current = match resolve_mut(&mut schema, &path) {
            Some(v) => v,
            None => continue,
        };

        let mut keys_to_keep: Vec<String> = Vec::new();
        let mut children_to_push: Vec<Vec<String>> = Vec::new();

        if let Some(properties) = current.get("properties").and_then(|v| v.as_object()) {
            for (prop_key, prop_value) in properties {
                let base_key = to_singular_without_numerical_suffix(prop_key);
                if mask.contains(&base_key) {
                    keys_to_keep.push(prop_key.clone());
                } else if let Some(re) = compiled.as_ref() {
                    if re.find(&base_key).is_some() {
                        keys_to_keep.push(prop_key.clone());
                    }
                }

                if recursive {
                    if is_object(prop_value) {
                        let mut p = path.clone();
                        p.push("properties".to_owned());
                        p.push(prop_key.clone());
                        children_to_push.push(p);
                    } else if is_array(prop_value) {
                        let mut p = path.clone();
                        p.push("properties".to_owned());
                        p.push(prop_key.clone());
                        p.push("items".to_owned());
                        children_to_push.push(p);
                    }
                }
            }
        }

        let keys_to_delete: Vec<String> = current
            .get("properties")
            .and_then(|v| v.as_object())
            .map(|props| {
                props
                    .keys()
                    .filter(|k| !keys_to_keep.contains(k))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        if let Some(properties) = current
            .get_mut("properties")
            .and_then(|v| v.as_object_mut())
        {
            for key in &keys_to_delete {
                properties.remove(key);
            }
        }

        if keys_to_keep.is_empty() {
            current.remove("required");
        } else if let Some(Value::Array(req)) = current.get_mut("required") {
            req.retain(|v| {
                v.as_str()
                    .map(|s| keys_to_keep.iter().any(|k| k == s))
                    .unwrap_or(false)
            });
            if req.is_empty() {
                current.remove("required");
            }
        }

        stack.extend(children_to_push);
    }

    prune_unreferenced_defs(&mut schema);
    Ok(schema)
}

/// Decompose a JSON schema by expanding array properties into individual properties.
pub fn decompose_schema(mut schema: Value) -> Value {
    let title = take_key(&mut schema, "title").unwrap_or(Value::Null);
    let defs = take_object(&mut schema, "$defs").unwrap_or_default();
    let schema_properties = take_object(&mut schema, "properties").unwrap_or_default();

    let n = schema_properties.len();
    let mut out_props: Map<String, Value> = Map::with_capacity(n);
    let mut out_required: Vec<Value> = Vec::with_capacity(n);

    for (prop_key, mut prop_value) in schema_properties {
        if is_plural(&prop_key) && is_array(&prop_value) {
            let singular_key = to_singular_without_numerical_suffix(&prop_key);

            // Pull items out so we can consume it without cloning.
            let items_schema = prop_value
                .as_object_mut()
                .and_then(|m| m.remove("items"));
            let mut individual_prop = prop_value;
            individual_prop.insert(
                "title".to_owned(),
                Value::String(titlecase(&singular_key)),
            );

            if let Some(items_schema) = items_schema {
                if let Value::Object(mut items_obj) = items_schema {
                    if let Some(item_type) = items_obj.remove("type") {
                        individual_prop.insert("type".to_owned(), item_type);
                    } else {
                        individual_prop.remove("type");
                        for (k, v) in items_obj {
                            individual_prop.insert(k, v);
                        }
                    }
                } else {
                    individual_prop.remove("type");
                }
            } else {
                individual_prop.insert(
                    "type".to_owned(),
                    Value::String("string".to_owned()),
                );
            }

            out_required.push(Value::String(singular_key.clone()));
            out_props.insert(singular_key, individual_prop);
        } else {
            out_required.push(Value::String(prop_key.clone()));
            out_props.insert(prop_key, prop_value);
        }
    }

    let mut result_schema = Value::Object(Map::with_capacity(6));
    result_schema.insert("additionalProperties".to_owned(), Value::Bool(false));
    if !defs.is_empty() {
        result_schema.insert("$defs".to_owned(), Value::Object(defs));
    }
    result_schema.insert("properties".to_owned(), Value::Object(out_props));
    result_schema.insert("required".to_owned(), Value::Array(out_required));
    result_schema.insert("title".to_owned(), title);
    result_schema.insert("type".to_owned(), Value::String("object".to_owned()));
    result_schema
}
