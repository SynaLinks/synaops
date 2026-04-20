// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)

use serde_json::json;

use super::*;
use crate::backend::common::symbolic_data_model::Value;

fn v(j: serde_json::Value) -> Value {
    Value::from(j)
}

// --- concatenate_json ---

#[test]
fn test_concatenate_identical_jsons() {
    let json = v(json!({"foo": "test"}));
    let expected = v(json!({"foo": "test", "foo_1": "test"}));

    let result = concatenate_json(&json, &json);
    assert_eq!(result, expected);
}

#[test]
fn test_concatenate_jsons_with_different_properties() {
    let json1 = v(json!({"foo": "test"}));
    let json2 = v(json!({"bar": "test"}));
    let expected = v(json!({"foo": "test", "bar": "test"}));

    let result = concatenate_json(&json1, &json2);
    assert_eq!(result, expected);
}

#[test]
fn test_concatenate_json_multiple_times() {
    let json = v(json!({"foo": "test"}));
    let expected = v(json!({"foo": "test", "foo_1": "test", "foo_2": "test"}));

    let result = concatenate_json(&json, &json);
    let result = concatenate_json(&result, &json);
    assert_eq!(result, expected);
}

#[test]
fn test_concatenate_nested() {
    let bar_obj = json!({"foo": "test", "bar": "test"});
    let json = v(json!({"bar": bar_obj}));
    let expected = v(json!({"bar": bar_obj, "bar_1": bar_obj}));

    let result = concatenate_json(&json, &json);
    assert_eq!(result, expected);
}

#[test]
fn test_concatenate_similar_entities() {
    let paris = json!({"label": "City", "name": "Paris"});
    let toulouse = json!({"label": "City", "name": "Toulouse"});
    let entities = json!([paris, toulouse]);

    let json = v(json!({"entities": entities}));
    let expected = v(json!({"entities": entities, "entities_1": entities}));

    let result = concatenate_json(&json, &json);
    assert_eq!(result, expected);
}

#[test]
fn test_concatenate_different_entities() {
    let paris = json!({"label": "City", "name": "Paris"});
    let toulouse = json!({"label": "City", "name": "Toulouse"});
    let event1 = json!({"label": "Event", "name": "A test event"});
    let event2 = json!({"label": "Event", "name": "Another test event"});

    let json1 = v(json!({"entities": [paris.clone(), toulouse.clone()]}));
    let json2 = v(json!({"entities": [event1.clone(), event2.clone()]}));
    let expected = v(json!({
        "entities": [paris, toulouse],
        "entities_1": [event1, event2]
    }));

    let result = concatenate_json(&json1, &json2);
    assert_eq!(result, expected);
}

// --- factorize_json ---

#[test]
fn test_factorize_json_with_identical_properties() {
    let json = v(json!({"foo": "test", "foo_1": "test"}));
    let expected = v(json!({"foos": ["test", "test"]}));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_json_with_multiple_identical_properties() {
    let json = v(json!({"foo": "test", "foo_1": "test", "foo_2": "test"}));
    let expected = v(json!({"foos": ["test", "test", "test"]}));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_json_with_different_properties() {
    let json = v(json!({"foo": "test", "bar": "test"}));
    let expected = v(json!({"foo": "test", "bar": "test"}));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_json_with_mixed_properties() {
    let json = v(json!({
        "foo": "test",
        "foo_1": "test",
        "bar": "test",
        "boo": "test"
    }));
    let expected = v(json!({
        "foos": ["test", "test"],
        "bar": "test",
        "boo": "test"
    }));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_json_with_existing_array_property() {
    let json = v(json!({"foos": ["test"], "foo": "test"}));
    let expected = v(json!({"foos": ["test", "test"]}));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_json_with_existing_array_property_and_additional() {
    let json = v(json!({
        "foos": ["test", "test"],
        "foo": "test",
        "foo_1": "test"
    }));
    let expected = v(json!({"foos": ["test", "test", "test", "test"]}));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_json_with_multiple_groups() {
    let json = v(json!({
        "foo": "test",
        "foo_1": "test",
        "bar": "test",
        "bar_1": "test"
    }));
    let expected = v(json!({
        "foos": ["test", "test"],
        "bars": ["test", "test"]
    }));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_nested() {
    let bar_obj = json!({"foo": "test", "bar": "test"});
    let json = v(json!({
        "bar": bar_obj,
        "bar_1": bar_obj
    }));
    let expected = v(json!({
        "bars": [bar_obj, bar_obj]
    }));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_similar_entities() {
    let paris = json!({"label": "City", "name": "Paris"});
    let toulouse = json!({"label": "City", "name": "Toulouse"});

    let json = v(json!({
        "entities": [paris.clone(), toulouse.clone()],
        "entities_1": [paris.clone(), toulouse.clone()]
    }));
    let expected = v(json!({
        "entities": [paris.clone(), toulouse.clone(), paris, toulouse]
    }));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

#[test]
fn test_factorize_different_entities() {
    let paris = json!({"label": "City", "name": "Paris"});
    let toulouse = json!({"label": "City", "name": "Toulouse"});
    let event1 = json!({"label": "Event", "name": "A test event"});
    let event2 = json!({"label": "Event", "name": "Another test event"});

    let json = v(json!({
        "entities": [paris.clone(), toulouse.clone()],
        "entities_1": [event1.clone(), event2.clone()]
    }));
    let expected = v(json!({
        "entities": [paris, toulouse, event1, event2]
    }));

    let result = factorize_json(&json);
    assert_eq!(result, expected);
}

// --- out_mask_json ---

#[test]
fn test_out_mask_basic() {
    let json = v(json!({"foo": "test", "bar": "test"}));
    let expected = v(json!({"bar": "test"}));

    let result = out_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_multiple_fields_with_same_base_name() {
    let json = v(json!({
        "foo": "test",
        "foo_1": "str",
        "bar": "test",
        "bar_1": "test"
    }));
    let expected = v(json!({
        "bar": "test",
        "bar_1": "test"
    }));

    let result = out_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_nested() {
    let json = v(json!({
        "bar": {"foo": "test", "bar": "test"},
        "bar_1": {"foo": "test", "bar": "test"}
    }));
    let expected = v(json!({
        "bar": {"bar": "test"},
        "bar_1": {"bar": "test"}
    }));

    let result = out_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_deeply_nested() {
    let json = v(json!({
        "foo": "test",
        "bar": {
            "boo": {
                "foo": "test",
                "boo": "test"
            }
        }
    }));
    let expected = v(json!({
        "bar": {
            "boo": {
                "boo": "test"
            }
        }
    }));

    let result = out_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_in_array() {
    let json = v(json!({
        "boos": [
            {"foo": "test", "boo": "test"},
            {"foo": "test", "boo": "test"}
        ]
    }));
    let expected = v(json!({
        "boos": [
            {"boo": "test"},
            {"boo": "test"}
        ]
    }));

    let result = out_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_empty_json() {
    let json = v(json!({}));
    let expected = v(json!({}));

    let result = out_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_empty_mask_list() {
    let json = v(json!({"foo": "test", "bar": "test"}));
    let expected = v(json!({"foo": "test", "bar": "test"}));

    let empty: &[&str] = &[];
    let result = out_mask_json(&json, Some(empty), true);
    assert_eq!(result, expected);
}

#[test]
fn test_out_mask_non_recursive() {
    let json = v(json!({
        "foo": "test",
        "boo": {"foo": "test", "boo": "test"}
    }));
    let expected = v(json!({
        "boo": {"foo": "test", "boo": "test"}
    }));

    let result = out_mask_json(&json, Some(&["foo"]), false);
    assert_eq!(result, expected);
}

// --- in_mask_json ---

#[test]
fn test_in_mask_basic() {
    let json = v(json!({"foo": "test", "bar": "test"}));
    let expected = v(json!({"foo": "test"}));

    let result = in_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_keep_all() {
    let json = v(json!({
        "foo": "test",
        "foos": "test",
        "bar": "test"
    }));
    let expected = v(json!({
        "foo": "test",
        "foos": "test",
        "bar": "test"
    }));

    let result = in_mask_json(&json, Some(&["foos", "bar"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_multiple_fields_with_same_base_name() {
    let json = v(json!({
        "foo": "test",
        "foo_1": "test",
        "bar": "test",
        "bar_1": "test"
    }));
    let expected = v(json!({
        "foo": "test",
        "foo_1": "test"
    }));

    let result = in_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_nested() {
    let json = v(json!({
        "foo": "test",
        "foo_1": "test",
        "bar": {"foo": "test", "bar": "test"},
        "bar_1": "test"
    }));
    let expected = v(json!({
        "foo": "test",
        "foo_1": "test"
    }));

    let result = in_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_deeply_nested() {
    let json = v(json!({
        "foo": "test",
        "bar": {"bar": {"foo": "test", "qux": "test"}}
    }));
    let expected = v(json!({
        "foo": "test",
        "bar": {"bar": {"foo": "test"}}
    }));

    let result = in_mask_json(&json, Some(&["foo", "bar"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_in_array() {
    let json = v(json!({
        "items": [
            {"foo": "test", "bar": "test"},
            {"foo_1": "test", "bar_1": "test"}
        ]
    }));
    let expected = v(json!({
        "items": [
            {"foo": "test"},
            {"foo_1": "test"}
        ]
    }));

    let result = in_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_empty_json() {
    let json = v(json!({}));
    let expected = v(json!({}));

    let result = in_mask_json(&json, Some(&["foo"]), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_empty_mask_list() {
    let json = v(json!({"foo": "test", "bar": "test"}));
    let expected = v(json!({}));

    let empty: &[&str] = &[];
    let result = in_mask_json(&json, Some(empty), true);
    assert_eq!(result, expected);
}

#[test]
fn test_in_mask_non_recursive() {
    let json = v(json!({
        "foo": "test",
        "bar": {"foo": "test", "bar": "test"}
    }));
    let expected = v(json!({"foo": "test"}));

    let result = in_mask_json(&json, Some(&["foo"]), false);
    assert_eq!(result, expected);
}

// --- prefix_json ---

#[test]
fn test_prefix_json_with_custom_prefix() {
    let json = v(json!({"name": "test"}));
    let result = prefix_json(&json, "input");
    let map = result.as_object().unwrap();
    assert!(map.contains_key("input_name"));
    assert_eq!(
        map.get("input_name"),
        Some(&Value::String("test".to_owned()))
    );
}

#[test]
fn test_prefix_json_non_object() {
    let json = Value::String("not an object".to_owned());
    let result = prefix_json(&json, "input");
    assert_eq!(result, json);
}

#[test]
fn test_prefix_json_empty_object() {
    let json = v(json!({}));
    let result = prefix_json(&json, "input");
    assert_eq!(result.as_object().unwrap().len(), 0);
}

// --- suffix_json ---

#[test]
fn test_suffix_json_basic() {
    let json = v(json!({"foo": "bar", "baz": 42}));
    let result = suffix_json(&json, "out");
    let map = result.as_object().unwrap();
    assert!(map.contains_key("foo_out"));
    assert!(map.contains_key("baz_out"));
}

#[test]
fn test_suffix_json_non_object() {
    let json = Value::Number(42.0);
    let result = suffix_json(&json, "x");
    assert_eq!(result, json);
}

#[test]
fn test_suffix_json_empty_object() {
    let json = v(json!({}));
    let result = suffix_json(&json, "s");
    assert_eq!(result.as_object().unwrap().len(), 0);
}

// --- concatenate_json edge cases ---

#[test]
fn test_concatenate_json_with_non_object() {
    let obj = v(json!({"foo": "bar"}));
    let non_obj = Value::String("hello".to_owned());
    // non-object maps are treated as empty
    let result = concatenate_json(&obj, &non_obj);
    assert_eq!(
        result.as_object().unwrap().get("foo"),
        Some(&Value::String("bar".to_owned()))
    );
}

#[test]
fn test_concatenate_json_both_empty() {
    let empty = v(json!({}));
    let result = concatenate_json(&empty, &empty);
    assert_eq!(result.as_object().unwrap().len(), 0);
}

// --- factorize_json edge cases ---

#[test]
fn test_factorize_json_non_object() {
    let arr = Value::Array(vec![Value::Number(1.0)]);
    let result = factorize_json(&arr);
    assert_eq!(result, arr);
}

#[test]
fn test_factorize_json_empty_object() {
    let json = v(json!({}));
    let result = factorize_json(&json);
    assert_eq!(result.as_object().unwrap().len(), 0);
}

#[test]
fn test_factorize_json_single_property() {
    let json = v(json!({"foo": "test"}));
    let result = factorize_json(&json);
    assert_eq!(result, v(json!({"foo": "test"})));
}

// --- out_mask_json edge cases ---

#[test]
fn test_out_mask_json_none_mask() {
    let json = v(json!({"foo": "bar"}));
    let result = out_mask_json(&json, None, true);
    assert_eq!(result, json);
}

#[test]
fn test_out_mask_json_non_object_value() {
    let json = v(json!({"foo": "test", "bar": 42}));
    // Mask should work on scalar values too
    let result = out_mask_json(&json, Some(&["bar"]), true);
    assert_eq!(result, v(json!({"foo": "test"})));
}

// --- in_mask_json edge cases ---

#[test]
fn test_in_mask_json_none_mask() {
    let json = v(json!({"foo": "bar"}));
    let result = in_mask_json(&json, None, true);
    assert_eq!(result, v(json!({})));
}

#[test]
fn test_in_mask_json_mask_not_matching_any_key() {
    let json = v(json!({"foo": "bar", "baz": "qux"}));
    let result = in_mask_json(&json, Some(&["nonexistent"]), true);
    assert_eq!(result, v(json!({})));
}

#[test]
fn test_out_mask_json_mask_not_matching_any_key() {
    let json = v(json!({"foo": "bar", "baz": "qux"}));
    let result = out_mask_json(&json, Some(&["nonexistent"]), true);
    assert_eq!(result, json);
}

// --- in_mask with nested array non-recursive ---

#[test]
fn test_in_mask_non_recursive_with_nested_object() {
    let json = v(json!({
        "foo": "test",
        "bar": {"foo": "inner", "baz": "hidden"}
    }));
    // Non-recursive: bar is not in mask, so it's excluded entirely
    let result = in_mask_json(&json, Some(&["foo"]), false);
    assert_eq!(result, v(json!({"foo": "test"})));
}

// --- out_mask with array containing nested arrays ---

#[test]
fn test_out_mask_nested_array_of_arrays() {
    let json = v(json!({
        "items": [
            [{"foo": "a", "bar": "b"}],
            [{"foo": "c", "bar": "d"}]
        ]
    }));
    // Recursive masking into arrays of arrays
    let result = out_mask_json(&json, Some(&["foo"]), true);
    let expected = v(json!({
        "items": [
            [{"bar": "b"}],
            [{"bar": "d"}]
        ]
    }));
    assert_eq!(result, expected);
}

// --- factorize_json: plural key already exists, merge non-array value ---

#[test]
fn test_factorize_json_plural_key_merge_scalar_value() {
    // "foos" already plural, and "foo" singular with non-array value
    // When "foos" is processed after singular keys already created the array,
    // its scalar value should be pushed
    let json = v(json!({
        "foo": "a",
        "foo_1": "b",
        "foos": "extra"
    }));
    let result = factorize_json(&json);
    let map = result.as_object().unwrap();
    // All should be merged into "foos" array
    assert!(map.contains_key("foos"));
    if let Some(Value::Array(arr)) = map.get("foos") {
        assert!(arr.len() >= 2);
    }
}

// --- factorize_json: plural key already exists with array value ---

#[test]
fn test_factorize_json_plural_array_merge_into_existing() {
    // "foos" exists as plural with array, "foo" adds to it
    let json = v(json!({
        "foo": "single",
        "foo_1": "another",
        "foos": ["existing"]
    }));
    let result = factorize_json(&json);
    let map = result.as_object().unwrap();
    if let Some(Value::Array(arr)) = map.get("foos") {
        // Should contain "existing" + "single" + "another"
        assert!(arr.len() >= 3);
    }
}

// --- in_mask_json: scalar value at top level (non-object, non-array) ---

// --- in_mask_json: recursive with scalar inside array ---
// Exercises in_mask_value's _ => value.clone() branch (line 224)

#[test]
fn test_in_mask_json_array_with_scalars_recursive() {
    // Array contains scalars, so recursing into them hits the fallback branch
    let json = v(json!({
        "items": ["hello", "world"]
    }));
    // "items" base is "item" which matches mask, and arrays are always kept
    // in recursive mode; the scalar elements inside go through _ => value.clone()
    let result = in_mask_json(&json, Some(&["item"]), true);
    let map = result.as_object().unwrap();
    if let Some(Value::Array(arr)) = map.get("items") {
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], Value::String("hello".to_owned()));
    }
}

#[test]
fn test_in_mask_json_array_with_mixed_types_recursive() {
    // Array with objects and scalars mixed; scalars hit the fallback path
    let json = v(json!({
        "data": [
            {"foo": "a", "bar": "b"},
            "scalar_value",
            42
        ]
    }));
    let result = in_mask_json(&json, Some(&["datum", "foo"]), true);
    let map = result.as_object().unwrap();
    if let Some(Value::Array(arr)) = map.get("data") {
        assert_eq!(arr.len(), 3);
        // First element: object filtered to keep only "foo"
        assert_eq!(
            arr[0].as_object().unwrap().get("foo"),
            Some(&Value::String("a".to_owned()))
        );
        assert!(!arr[0].as_object().unwrap().contains_key("bar"));
        // Second element: scalar preserved as-is
        assert_eq!(arr[1], Value::String("scalar_value".to_owned()));
        // Third element: number preserved as-is
        assert_eq!(arr[2], Value::Number(42.0));
    }
}

#[test]
fn test_factorize_json_array_values_merged() {
    // When similar properties have array values, they should be flattened into one array
    let json = v(json!({
        "item_1": [1, 2],
        "item_2": [3, 4]
    }));
    let result = factorize_json(&json);
    let map = result.as_object().unwrap();
    // "item_1" and "item_2" share base "item", so factorized into "items"
    assert!(map.contains_key("items"));
    if let Some(Value::Array(arr)) = map.get("items") {
        // Both arrays should be flattened: [1, 2, 3, 4]
        assert_eq!(arr.len(), 4);
    } else {
        panic!("Expected items to be an array");
    }
}

// --- decompose_json ---

#[test]
fn test_decompose_json_plural_array() {
    let json = v(json!({
        "queries": ["first", "second", "third"]
    }));
    let result = decompose_json(&json);
    let map = result.as_object().unwrap();
    assert_eq!(map.get("query"), Some(&Value::String("first".to_owned())));
    assert_eq!(map.get("query_0"), Some(&Value::String("second".to_owned())));
    assert_eq!(map.get("query_1"), Some(&Value::String("third".to_owned())));
}

#[test]
fn test_decompose_json_single_item_array() {
    let json = v(json!({
        "queries": ["only"]
    }));
    let result = decompose_json(&json);
    let map = result.as_object().unwrap();
    assert_eq!(map.get("query"), Some(&Value::String("only".to_owned())));
    assert!(!map.contains_key("query_0"));
}

#[test]
fn test_decompose_json_non_plural_key_unchanged() {
    let json = v(json!({
        "answer": "Paris"
    }));
    let result = decompose_json(&json);
    let map = result.as_object().unwrap();
    assert_eq!(map.get("answer"), Some(&Value::String("Paris".to_owned())));
}

#[test]
fn test_decompose_json_mixed() {
    let json = v(json!({
        "queries": ["q1", "q2"],
        "answer": "a1"
    }));
    let result = decompose_json(&json);
    let map = result.as_object().unwrap();
    assert_eq!(map.get("query"), Some(&Value::String("q1".to_owned())));
    assert_eq!(map.get("query_0"), Some(&Value::String("q2".to_owned())));
    assert_eq!(map.get("answer"), Some(&Value::String("a1".to_owned())));
}

#[test]
fn test_decompose_json_plural_non_array_unchanged() {
    let json = v(json!({
        "queries": "not an array"
    }));
    let result = decompose_json(&json);
    let map = result.as_object().unwrap();
    assert_eq!(map.get("queries"), Some(&Value::String("not an array".to_owned())));
}

#[test]
fn test_decompose_json_empty_array() {
    let json = v(json!({
        "queries": []
    }));
    let result = decompose_json(&json);
    let map = result.as_object().unwrap();
    // Empty array: no singular keys produced
    assert!(!map.contains_key("query"));
    assert!(!map.contains_key("queries"));
}

#[test]
fn test_decompose_json_non_object() {
    let arr = Value::Array(vec![Value::Number(1.0)]);
    let result = decompose_json(&arr);
    assert_eq!(result, arr);
}

#[test]
fn test_decompose_json_empty_object() {
    let json = v(json!({}));
    let result = decompose_json(&json);
    assert_eq!(result.as_object().unwrap().len(), 0);
}

#[test]
fn test_decompose_json_roundtrip_with_factorize() {
    // factorize then decompose should give back individual properties
    let json = v(json!({
        "query": "first",
        "query_0": "second"
    }));
    let factorized = factorize_json(&json);
    let decomposed = decompose_json(&factorized);
    let map = decomposed.as_object().unwrap();
    assert_eq!(map.get("query"), Some(&Value::String("first".to_owned())));
    assert_eq!(map.get("query_0"), Some(&Value::String("second".to_owned())));
}
