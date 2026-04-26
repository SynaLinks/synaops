// License Apache 2.0: (c) 2025 Yoan Sallami (Synalinks Team)

use serde_json::json;

use super::*;
use crate::backend::common::symbolic_data_model::Value;

/// Convert a serde_json::Value into our Value type.
fn v(j: serde_json::Value) -> Value {
    Value::from(j)
}

// =========================================================================
// JsonSchemaConcatenateTest
// =========================================================================

#[test]
fn test_concatenate_identical_schemas() {
    // class Input(DataModel): foo: str
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));

    // class Result(DataModel): foo: str; foo_1: str
    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"}
        },
        "required": ["foo", "foo_1"],
        "title": "Result",
        "type": "object"
    }));

    let result = concatenate_schema(&schema, &schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_concatenate_schemas_with_different_properties() {
    let schema1 = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input1",
        "type": "object"
    }));

    let schema2 = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["bar"],
        "title": "Input2",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = concatenate_schema(&schema1, &schema2);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_concatenate_similar_entities() {
    // class City(DataModel): label: Literal["City"]; name: str
    // class Cities(DataModel): entities: List[City]
    let cities_schema = v(json!({
        "$defs": {
            "City": {
                "additionalProperties": false,
                "properties": {
                    "label": {"const": "City", "title": "Label", "type": "string"},
                    "name": {"title": "Name", "type": "string"}
                },
                "required": ["label", "name"],
                "title": "City",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            }
        },
        "required": ["entities"],
        "title": "Cities",
        "type": "object"
    }));

    // class Result(DataModel): entities: List[City]; entities_1: List[City]
    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            },
            "entities_1": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities 1",
                "type": "array"
            }
        },
        "required": ["entities", "entities_1"],
        "title": "Result",
        "type": "object"
    }));

    let result = concatenate_schema(&cities_schema, &cities_schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_concatenate_different_entities() {
    // Cities schema with City $def
    let cities_schema = v(json!({
        "$defs": {
            "City": {
                "additionalProperties": false,
                "properties": {
                    "label": {"const": "City", "title": "Label", "type": "string"},
                    "name": {"title": "Name", "type": "string"}
                },
                "required": ["label", "name"],
                "title": "City",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            }
        },
        "required": ["entities"],
        "title": "Cities",
        "type": "object"
    }));

    // Events schema with Event $def
    let events_schema = v(json!({
        "$defs": {
            "Event": {
                "additionalProperties": false,
                "properties": {
                    "label": {"const": "Event", "title": "Label", "type": "string"},
                    "name": {"title": "Name", "type": "string"}
                },
                "required": ["label", "name"],
                "title": "Event",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/Event"},
                "title": "Entities",
                "type": "array"
            }
        },
        "required": ["entities"],
        "title": "Events",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            },
            "entities_1": {
                "items": {"$ref": "#/$defs/Event"},
                "title": "Entities 1",
                "type": "array"
            }
        },
        "required": ["entities", "entities_1"],
        "title": "Result",
        "type": "object"
    }));

    let result = concatenate_schema(&cities_schema, &events_schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_concatenate_schema_multiple_times() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "foo_2": {"title": "Foo 2", "type": "string"}
        },
        "required": ["foo", "foo_1", "foo_2"],
        "title": "Result",
        "type": "object"
    }));

    let result = concatenate_schema(&schema, &schema);
    let result = concatenate_schema(&result, &schema);
    assert!(is_schema_equal(&result, &expected));
}

// =========================================================================
// JsonSchemaFactoriseTest
// =========================================================================

#[test]
fn test_factorize_schema_with_identical_properties() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"}
        },
        "required": ["foo", "foo_1"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            }
        },
        "required": ["foos"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_schema_with_multiple_identical_properties() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "foo_2": {"title": "Foo 2", "type": "string"}
        },
        "required": ["foo", "foo_1", "foo_2"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            }
        },
        "required": ["foos"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_schema_with_mixed_properties() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "foo_1", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            },
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foos", "bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_schema_with_existing_array_property() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            },
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foos", "foo"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            }
        },
        "required": ["foos"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_schema_with_existing_array_property_and_additional_properties() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            },
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"}
        },
        "required": ["foos", "foo", "foo_1"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            }
        },
        "required": ["foos"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_schema_with_multiple_groups_of_properties() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"title": "Bar", "type": "string"},
            "bar_1": {"title": "Bar 1", "type": "string"}
        },
        "required": ["foo", "foo_1", "bar", "bar_1"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foos": {
                "items": {"type": "string"},
                "title": "Foos",
                "type": "array"
            },
            "bars": {
                "items": {"type": "string"},
                "title": "Bars",
                "type": "array"
            }
        },
        "required": ["foos", "bars"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_similar_entities() {
    // entities: List[City], entities_1: List[City] (same $ref)
    let schema = v(json!({
        "$defs": {
            "City": {
                "additionalProperties": false,
                "properties": {
                    "label": {"const": "City", "title": "Label", "type": "string"},
                    "name": {"title": "Name", "type": "string"}
                },
                "required": ["label", "name"],
                "title": "City",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            },
            "entities_1": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities 1",
                "type": "array"
            }
        },
        "required": ["entities", "entities_1"],
        "title": "Input",
        "type": "object"
    }));

    // Result: entities: List[City] (single array, same items)
    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            }
        },
        "required": ["entities"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_factorize_different_entities() {
    // entities: List[City], entities_1: List[Event] (different $refs)
    let schema = v(json!({
        "$defs": {
            "City": {
                "additionalProperties": false,
                "properties": {
                    "label": {"const": "City", "title": "Label", "type": "string"},
                    "name": {"title": "Name", "type": "string"}
                },
                "required": ["label", "name"],
                "title": "City",
                "type": "object"
            },
            "Event": {
                "additionalProperties": false,
                "properties": {
                    "label": {"const": "Event", "title": "Label", "type": "string"},
                    "name": {"title": "Name", "type": "string"}
                },
                "required": ["label", "name"],
                "title": "Event",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entities",
                "type": "array"
            },
            "entities_1": {
                "items": {"$ref": "#/$defs/Event"},
                "title": "Entities 1",
                "type": "array"
            }
        },
        "required": ["entities", "entities_1"],
        "title": "Input",
        "type": "object"
    }));

    // Result: entities: List[Union[City, Event]] => anyOf
    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "entities": {
                "items": {
                    "anyOf": [
                        {"$ref": "#/$defs/City"},
                        {"$ref": "#/$defs/Event"}
                    ]
                },
                "title": "Entities",
                "type": "array"
            }
        },
        "required": ["entities"],
        "title": "Result",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    assert!(is_schema_equal(&result, &expected));
}

// =========================================================================
// JsonSchemaOutMaskTest
// =========================================================================

#[test]
fn test_out_mask_basic() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_out_mask_multiple_fields_with_same_base_name() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"title": "Bar", "type": "string"},
            "bar_1": {"title": "Bar 1", "type": "string"}
        },
        "required": ["foo", "foo_1", "bar", "bar_1"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"title": "Bar", "type": "string"},
            "bar_1": {"title": "Bar 1", "type": "string"}
        },
        "required": ["bar", "bar_1"],
        "title": "Result",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_out_mask_nested() {
    // BarObject has foo and bar; Input has foo, foo_1, bar: BarObject
    let schema = v(json!({
        "$defs": {
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"}
                },
                "required": ["foo", "bar"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "foo_1", "bar"],
        "title": "Input",
        "type": "object"
    }));

    // After masking "foo": top-level foo/foo_1 removed,
    // BarObject.foo also removed (recursive)
    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));

    // Verify BarObject in $defs was also masked
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(!bar_props.contains_key("foo"));
    assert!(bar_props.contains_key("bar"));
}

#[test]
fn test_out_mask_deeply_nested() {
    // BooObject has foo and boo; BarObject has boo: BooObject; Input has foo, bar: BarObject
    let schema = v(json!({
        "$defs": {
            "BooObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "boo": {"title": "Boo", "type": "string"}
                },
                "required": ["foo", "boo"],
                "title": "BooObject",
                "type": "object"
            },
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "boo": {"$ref": "#/$defs/BooObject"}
                },
                "required": ["boo"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));

    // Verify BooObject.foo was removed recursively
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let boo_obj = defs.get("BooObject").unwrap();
    let boo_props = boo_obj.get("properties").unwrap().as_object().unwrap();
    assert!(!boo_props.contains_key("foo"));
    assert!(boo_props.contains_key("boo"));
}

#[test]
fn test_out_mask_array() {
    // BarObject has foo and bar; Input has bars: List[BarObject]
    let schema = v(json!({
        "$defs": {
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"}
                },
                "required": ["foo", "bar"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "bars": {
                "items": {"$ref": "#/$defs/BarObject"},
                "title": "Bars",
                "type": "array"
            }
        },
        "required": ["bars"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bars": {
                "items": {"$ref": "#/$defs/BarObject"},
                "title": "Bars",
                "type": "array"
            }
        },
        "required": ["bars"],
        "title": "Result",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));

    // Verify BarObject.foo was removed via array items recursion
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(!bar_props.contains_key("foo"));
    assert!(bar_props.contains_key("bar"));
}

#[test]
fn test_out_mask_empty_schema() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {},
        "title": "Input",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &schema));
}

#[test]
fn test_out_mask_empty_mask_list() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let empty_mask: &[&str] = &[];
    let result = out_mask_schema(&schema, Some(empty_mask), true);
    assert!(is_schema_equal(&result, &schema));
}

#[test]
fn test_out_mask_non_recursive() {
    // BarObject has foo and bar; Input has foo, foo_1, bar: BarObject
    // Non-recursive: only top-level foo/foo_1 removed, BarObject.foo stays
    let schema = v(json!({
        "$defs": {
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"}
                },
                "required": ["foo", "bar"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "foo_1", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), false);
    assert!(is_schema_equal(&result, &expected));

    // BarObject.foo should still be present (non-recursive)
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(bar_props.contains_key("foo"));
    assert!(bar_props.contains_key("bar"));
}

// =========================================================================
// JsonSchemaInMaskTest
// =========================================================================

#[test]
fn test_in_mask_basic() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Result",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_in_mask_multiple_fields_with_same_base_name() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"title": "Bar", "type": "string"},
            "bar_1": {"title": "Bar 1", "type": "string"}
        },
        "required": ["foo", "foo_1", "bar", "bar_1"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"}
        },
        "required": ["foo", "foo_1"],
        "title": "Result",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_in_mask_nested() {
    // BarObject has foo, bar, boo; Input has foo, foo_1, bar: BarObject, boo
    // mask=["foo", "bar"] keeps foo/foo_1/bar at top level,
    // and recursively keeps foo/bar inside BarObject
    let schema = v(json!({
        "$defs": {
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"},
                    "boo": {"title": "Boo", "type": "string"}
                },
                "required": ["foo", "bar", "boo"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"},
            "boo": {"title": "Boo", "type": "string"}
        },
        "required": ["foo", "foo_1", "bar", "boo"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "foo_1", "bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo", "bar"]), true);
    assert!(is_schema_equal(&result, &expected));

    // Verify BarObject was also masked: boo removed, foo+bar kept
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(bar_props.contains_key("foo"));
    assert!(bar_props.contains_key("bar"));
    assert!(!bar_props.contains_key("boo"));
}

#[test]
fn test_in_mask_deeply_nested() {
    // BooObject has foo and boo; BarObject has boo: BooObject; Input has foo, bar: BarObject
    // mask=["foo", "bar"] => keep foo at top, keep bar (BarObject),
    // inside BooObject keep foo only (boo is not in mask? actually "bar" matches "bar")
    // Actually the mask is about base keys. "boo" is not in mask, so BooObject.boo gets removed.
    let schema = v(json!({
        "$defs": {
            "BooObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "boo": {"title": "Boo", "type": "string"}
                },
                "required": ["foo", "boo"],
                "title": "BooObject",
                "type": "object"
            },
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "boo": {"$ref": "#/$defs/BooObject"}
                },
                "required": ["boo"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo", "bar"]), true);
    assert!(is_schema_equal(&result, &expected));

    // BarObject's only property "boo" was removed (not in mask),
    // so BarObject is now empty. BooObject had "boo" removed,
    // keeping only "foo". But since BarObject no longer references
    // BooObject, the $defs cleanup removes BooObject.
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(bar_props.is_empty());

    // BooObject reference was removed during cleanup (no longer referenced)
    assert!(!defs.contains_key("BooObject"));
}

#[test]
fn test_in_mask_array() {
    // BarObject has foo and bar; Input has bars: List[BarObject]
    // mask=["bar"] keeps bars (base "bar"), and inside BarObject keeps bar only
    let schema = v(json!({
        "$defs": {
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"}
                },
                "required": ["foo", "bar"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "bars": {
                "items": {"$ref": "#/$defs/BarObject"},
                "title": "Bars",
                "type": "array"
            }
        },
        "required": ["bars"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "bars": {
                "items": {"$ref": "#/$defs/BarObject"},
                "title": "Bars",
                "type": "array"
            }
        },
        "required": ["bars"],
        "title": "Result",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["bar"]), true);
    assert!(is_schema_equal(&result, &expected));

    // BarObject should only have bar (foo removed)
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(!bar_props.contains_key("foo"));
    assert!(bar_props.contains_key("bar"));
}

#[test]
fn test_in_mask_empty_schema() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {},
        "title": "Input",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo"]), true);
    assert!(is_schema_equal(&result, &schema));
}

#[test]
fn test_in_mask_empty_mask_list() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {},
        "title": "Input",
        "type": "object"
    }));

    let empty_mask: &[&str] = &[];
    let result = in_mask_schema(&schema, Some(empty_mask), true);
    assert!(is_schema_equal(&result, &expected));
}

#[test]
fn test_in_mask_non_recursive() {
    // Non-recursive: only top-level filtering, nested BarObject untouched
    let schema = v(json!({
        "$defs": {
            "BarObject": {
                "additionalProperties": false,
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"},
                    "boo": {"title": "Boo", "type": "string"}
                },
                "required": ["foo", "bar", "boo"],
                "title": "BarObject",
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"},
            "boo": {"title": "Boo", "type": "string"}
        },
        "required": ["foo", "foo_1", "bar", "boo"],
        "title": "Input",
        "type": "object"
    }));

    let expected = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"},
            "bar": {"$ref": "#/$defs/BarObject"}
        },
        "required": ["foo", "foo_1", "bar"],
        "title": "Result",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo", "bar"]), false);
    assert!(is_schema_equal(&result, &expected));

    // BarObject should be untouched (non-recursive) — all 3 properties remain
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    let bar_obj = defs.get("BarObject").unwrap();
    let bar_props = bar_obj.get("properties").unwrap().as_object().unwrap();
    assert!(bar_props.contains_key("foo"));
    assert!(bar_props.contains_key("bar"));
    assert!(bar_props.contains_key("boo"));
}

// =========================================================================
// JsonSchemaContainsTest
// =========================================================================

#[test]
fn test_contains_same_schema() {
    let schema1 = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input1",
        "type": "object"
    }));

    let schema2 = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input2",
        "type": "object"
    }));

    assert!(contains_schema(&schema1, &schema2));
}

#[test]
fn test_contains_subset_schema() {
    let schema1 = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input1",
        "type": "object"
    }));

    let schema2 = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input2",
        "type": "object"
    }));

    assert!(contains_schema(&schema1, &schema2));
}

#[test]
fn test_contains_different_schema() {
    let schema1 = v(json!({
        "additionalProperties": false,
        "properties": {
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["bar"],
        "title": "Input1",
        "type": "object"
    }));

    let schema2 = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input2",
        "type": "object"
    }));

    assert!(!contains_schema(&schema1, &schema2));
}

// =========================================================================
// Additional coverage tests
// =========================================================================

// --- standardize_schema ---

#[test]
fn test_standardize_schema_passthrough() {
    let schema = v(json!({"type": "object", "properties": {"a": {"type": "string"}}}));
    let result = standardize_schema(schema.clone());
    assert_eq!(result, schema);
}

// --- is_object / is_array ---

#[test]
fn test_is_object_true() {
    let schema = v(json!({"type": "object"}));
    assert!(is_object(&schema));
}

#[test]
fn test_is_object_false() {
    let schema = v(json!({"type": "array"}));
    assert!(!is_object(&schema));
}

#[test]
fn test_is_object_no_type() {
    let schema = v(json!({"properties": {}}));
    assert!(!is_object(&schema));
}

#[test]
fn test_is_array_true() {
    let schema = v(json!({"type": "array"}));
    assert!(is_array(&schema));
}

#[test]
fn test_is_array_false() {
    let schema = v(json!({"type": "object"}));
    assert!(!is_array(&schema));
}

#[test]
fn test_is_array_no_type() {
    let schema = v(json!({"items": {"type": "string"}}));
    assert!(!is_array(&schema));
}

// --- is_schema_equal ---

#[test]
fn test_is_schema_equal_same() {
    let schema = v(json!({
        "properties": {"foo": {"type": "string"}},
        "type": "object"
    }));
    assert!(is_schema_equal(&schema, &schema));
}

#[test]
fn test_is_schema_equal_different_properties() {
    let s1 = v(json!({"properties": {"foo": {"type": "string"}}}));
    let s2 = v(json!({"properties": {"bar": {"type": "string"}}}));
    assert!(!is_schema_equal(&s1, &s2));
}

#[test]
fn test_is_schema_equal_different_count() {
    let s1 = v(json!({"properties": {"foo": {"type": "string"}, "bar": {"type": "string"}}}));
    let s2 = v(json!({"properties": {"foo": {"type": "string"}}}));
    assert!(!is_schema_equal(&s1, &s2));
}

#[test]
fn test_is_schema_equal_no_properties() {
    let s1 = v(json!({"type": "object"}));
    let s2 = v(json!({"type": "object"}));
    assert!(is_schema_equal(&s1, &s2));
}

// --- prefix_schema ---

#[test]
fn test_prefix_schema_basic() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "name": {"title": "Name", "type": "string"}
        },
        "required": ["name"],
        "title": "Input",
        "type": "object"
    }));

    let result = prefix_schema(&schema, "input");
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("input_name"));
    assert!(!props.contains_key("name"));
    let title = props
        .get("input_name")
        .unwrap()
        .get("title")
        .unwrap()
        .as_str()
        .unwrap();
    assert_eq!(title, "Input Name");
}

#[test]
fn test_prefix_schema_updates_required() {
    let schema = v(json!({
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "required": ["foo"],
        "type": "object"
    }));

    let result = prefix_schema(&schema, "pre");
    let required = result.get("required").unwrap().as_array().unwrap();
    assert_eq!(required.len(), 1);
    assert_eq!(required[0].as_str(), Some("pre_foo"));
}

#[test]
fn test_prefix_schema_no_existing_title() {
    let schema = v(json!({
        "properties": {"bar": {"type": "string"}},
        "type": "object"
    }));

    let result = prefix_schema(&schema, "x");
    let props = result.get("properties").unwrap().as_object().unwrap();
    let title = props
        .get("x_bar")
        .unwrap()
        .get("title")
        .unwrap()
        .as_str()
        .unwrap();
    assert_eq!(title, "X Bar");
}

// --- suffix_schema ---

#[test]
fn test_suffix_schema_basic() {
    let schema = v(json!({
        "properties": {
            "name": {"title": "Name", "type": "string"}
        },
        "required": ["name"],
        "type": "object"
    }));

    let result = suffix_schema(&schema, "out");
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("name_out"));
    assert!(!props.contains_key("name"));
    let title = props
        .get("name_out")
        .unwrap()
        .get("title")
        .unwrap()
        .as_str()
        .unwrap();
    assert_eq!(title, "Name Out");
}

#[test]
fn test_suffix_schema_updates_required() {
    let schema = v(json!({
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "required": ["foo"],
        "type": "object"
    }));

    let result = suffix_schema(&schema, "1");
    let required = result.get("required").unwrap().as_array().unwrap();
    assert_eq!(required.len(), 1);
    assert_eq!(required[0].as_str(), Some("foo_1"));
}

// --- contains_schema edge cases ---

#[test]
fn test_contains_schema_no_properties_in_schema1() {
    let s1 = v(json!({"type": "object"}));
    let s2 = v(json!({"properties": {"foo": {"type": "string"}}}));
    assert!(!contains_schema(&s1, &s2));
}

#[test]
fn test_contains_schema_no_properties_in_schema2() {
    let s1 = v(json!({"properties": {"foo": {"type": "string"}}}));
    let s2 = v(json!({"type": "object"}));
    assert!(!contains_schema(&s1, &s2));
}

#[test]
fn test_contains_schema_both_no_properties() {
    let s1 = v(json!({"type": "object"}));
    let s2 = v(json!({"type": "object"}));
    assert!(!contains_schema(&s1, &s2));
}

// --- concatenate_schema edge cases ---

#[test]
fn test_concatenate_schema_empty_properties() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {},
        "title": "Empty",
        "type": "object"
    }));
    let result = concatenate_schema(&schema, &schema);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

#[test]
fn test_concatenate_schema_no_required() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "title": "Input",
        "type": "object"
    }));
    let result = concatenate_schema(&schema, &schema);
    // Without required in input, the result should have no required entries
    let required = result.get("required").unwrap().as_array().unwrap();
    assert!(required.is_empty());
}

// --- factorize_schema edge cases ---

#[test]
fn test_factorize_schema_no_duplicates() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "integer"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("foo"));
    assert!(props.contains_key("bar"));
    assert_eq!(props.len(), 2);
}

#[test]
fn test_factorize_schema_empty() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {},
        "title": "Empty",
        "type": "object"
    }));
    let result = factorize_schema(&schema);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

// --- out_mask_schema with None mask ---

#[test]
fn test_out_mask_schema_none_mask() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));
    let result = out_mask_schema(&schema, None, true);
    assert!(is_schema_equal(&result, &schema));
}

// --- in_mask_schema with None mask ---

#[test]
fn test_in_mask_schema_none_mask() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));
    let result = in_mask_schema(&schema, None, true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

// --- out_mask_schema mask not matching ---

#[test]
fn test_out_mask_schema_no_match() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));
    let result = out_mask_schema(&schema, Some(&["nonexistent"]), true);
    assert!(is_schema_equal(&result, &schema));
}

// --- in_mask_schema mask not matching ---

#[test]
fn test_in_mask_schema_no_match() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));
    let result = in_mask_schema(&schema, Some(&["nonexistent"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

// --- concatenate_schema preserves title from schema1 ---

#[test]
fn test_concatenate_schema_preserves_title() {
    let schema1 = v(json!({
        "properties": {"a": {"title": "A", "type": "string"}},
        "required": ["a"],
        "title": "MyTitle",
        "type": "object"
    }));
    let schema2 = v(json!({
        "properties": {"b": {"title": "B", "type": "string"}},
        "required": ["b"],
        "title": "OtherTitle",
        "type": "object"
    }));
    let result = concatenate_schema(&schema1, &schema2);
    assert_eq!(result.get("title").unwrap().as_str(), Some("MyTitle"));
}

// =========================================================================
// Extended coverage tests
// =========================================================================

// --- concatenate_schema: one-sided $defs merging ---

#[test]
fn test_concatenate_schema_defs_only_in_first() {
    let schema1 = v(json!({
        "$defs": {
            "City": {
                "properties": {"name": {"type": "string"}},
                "type": "object"
            }
        },
        "properties": {
            "entity": {
                "items": {"$ref": "#/$defs/City"},
                "type": "array"
            }
        },
        "required": ["entity"],
        "title": "S1",
        "type": "object"
    }));
    let schema2 = v(json!({
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "S2",
        "type": "object"
    }));

    let result = concatenate_schema(&schema1, &schema2);
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    assert!(defs.contains_key("City"));
}

#[test]
fn test_concatenate_schema_defs_only_in_second() {
    let schema1 = v(json!({
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "S1",
        "type": "object"
    }));
    let schema2 = v(json!({
        "$defs": {
            "Event": {
                "properties": {"name": {"type": "string"}},
                "type": "object"
            }
        },
        "properties": {
            "entity": {
                "items": {"$ref": "#/$defs/Event"},
                "type": "array"
            }
        },
        "required": ["entity"],
        "title": "S2",
        "type": "object"
    }));

    let result = concatenate_schema(&schema1, &schema2);
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    assert!(defs.contains_key("Event"));
}

// --- concatenate_schema: required field logic branches ---

#[test]
fn test_concatenate_schema_required_only_in_first() {
    let schema1 = v(json!({
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "required": ["foo"],
        "title": "S1",
        "type": "object"
    }));
    // schema2 has same property name but no required list
    let schema2 = v(json!({
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "title": "S2",
        "type": "object"
    }));

    let result = concatenate_schema(&schema1, &schema2);
    let required = result
        .get("required")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_owned()))
        .collect::<Vec<String>>();
    // "foo" from schema1 is required
    assert!(required.contains(&"foo".to_owned()));
}

#[test]
fn test_concatenate_schema_required_only_in_second() {
    let schema1 = v(json!({
        "properties": {"foo": {"title": "Foo", "type": "string"}},
        "title": "S1",
        "type": "object"
    }));
    let schema2 = v(json!({
        "properties": {"bar": {"title": "Bar", "type": "string"}},
        "required": ["bar"],
        "title": "S2",
        "type": "object"
    }));

    let result = concatenate_schema(&schema1, &schema2);
    let required = result
        .get("required")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_owned()))
        .collect::<Vec<String>>();
    assert!(required.contains(&"bar".to_owned()));
    // "foo" has no required list, should not be added
    assert!(!required.contains(&"foo".to_owned()));
}

// --- prefix_schema / suffix_schema with empty properties ---

#[test]
fn test_prefix_schema_empty_properties() {
    let schema = v(json!({
        "properties": {},
        "type": "object"
    }));
    let result = prefix_schema(&schema, "pre");
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
    let required = result.get("required").unwrap().as_array().unwrap();
    assert!(required.is_empty());
}

#[test]
fn test_suffix_schema_empty_properties() {
    let schema = v(json!({
        "properties": {},
        "type": "object"
    }));
    let result = suffix_schema(&schema, "suf");
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

// --- out_mask_schema: recursive with inline object properties ---

#[test]
fn test_out_mask_schema_recursive_inline_object() {
    // Schema with an inline nested object (type=object with properties inside)
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "nested": {
                "title": "Nested",
                "type": "object",
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"}
                },
                "required": ["foo", "bar"]
            }
        },
        "required": ["foo", "nested"],
        "title": "Input",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    // Top-level "foo" removed
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(!props.contains_key("foo"));
    assert!(props.contains_key("nested"));
    // Inside nested: "foo" should also be removed recursively
    let nested = props.get("nested").unwrap();
    let nested_props = nested.get("properties").unwrap().as_object().unwrap();
    assert!(!nested_props.contains_key("foo"));
    assert!(nested_props.contains_key("bar"));
}

// --- out_mask_schema: recursive with inline array properties ---

#[test]
fn test_out_mask_schema_recursive_inline_array() {
    // Schema with an inline array (type=array with items having properties)
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "items": {
                "title": "Items",
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "foo": {"title": "Foo", "type": "string"},
                        "bar": {"title": "Bar", "type": "string"}
                    },
                    "required": ["foo", "bar"]
                }
            }
        },
        "required": ["items"],
        "title": "Input",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    let items_prop = props.get("items").unwrap();
    let items_inner = items_prop.get("items").unwrap();
    let inner_props = items_inner.get("properties").unwrap().as_object().unwrap();
    assert!(!inner_props.contains_key("foo"));
    assert!(inner_props.contains_key("bar"));
}

// --- in_mask_schema: recursive with inline object properties ---

#[test]
fn test_in_mask_schema_recursive_inline_object() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "nested": {
                "title": "Nested",
                "type": "object",
                "properties": {
                    "foo": {"title": "Foo", "type": "string"},
                    "bar": {"title": "Bar", "type": "string"},
                    "baz": {"title": "Baz", "type": "string"}
                },
                "required": ["foo", "bar", "baz"]
            }
        },
        "required": ["foo", "nested"],
        "title": "Input",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo", "nested"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("foo"));
    assert!(props.contains_key("nested"));
    // Inside nested: only "foo" should be kept (and "nested" matches nothing inside)
    let nested = props.get("nested").unwrap();
    let nested_props = nested.get("properties").unwrap().as_object().unwrap();
    assert!(nested_props.contains_key("foo"));
    assert!(nested_props.contains_key("nested") || !nested_props.contains_key("bar"));
    assert!(!nested_props.contains_key("baz"));
}

// --- in_mask_schema: recursive with inline array properties ---

#[test]
fn test_in_mask_schema_recursive_inline_array() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "items": {
                "title": "Items",
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "foo": {"title": "Foo", "type": "string"},
                        "bar": {"title": "Bar", "type": "string"},
                        "baz": {"title": "Baz", "type": "string"}
                    },
                    "required": ["foo", "bar", "baz"]
                }
            }
        },
        "required": ["items"],
        "title": "Input",
        "type": "object"
    }));

    // mask=["item", "foo"] — "item" is singular of "items", so items is kept
    let result = in_mask_schema(&schema, Some(&["item", "foo"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("items"));
    // Inside items.items: only "foo" should be kept
    let items_prop = props.get("items").unwrap();
    let items_inner = items_prop.get("items").unwrap();
    let inner_props = items_inner.get("properties").unwrap().as_object().unwrap();
    assert!(inner_props.contains_key("foo"));
    assert!(!inner_props.contains_key("bar"));
    assert!(!inner_props.contains_key("baz"));
}

// --- in_mask_schema: required retained for some keys ---

#[test]
fn test_in_mask_schema_required_partially_retained() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"},
            "baz": {"title": "Baz", "type": "string"}
        },
        "required": ["foo", "bar", "baz"],
        "title": "Input",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo", "bar"]), true);
    let required = result
        .get("required")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_owned()))
        .collect::<Vec<String>>();
    assert!(required.contains(&"foo".to_owned()));
    assert!(required.contains(&"bar".to_owned()));
    assert!(!required.contains(&"baz".to_owned()));
}

// --- factorize_schema: array + non-array similar properties ---

#[test]
fn test_factorize_schema_array_with_non_array_similar() {
    // entity (array type) + entity_1 (string type, not an array)
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "entity": {
                "items": {"type": "string"},
                "title": "Entity",
                "type": "array"
            },
            "entity_1": {"title": "Entity 1", "type": "string"}
        },
        "required": ["entity", "entity_1"],
        "title": "Input",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    // Should produce "entities" array
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("entities"));
}

// --- factorize_schema with $defs cleanup ---

#[test]
fn test_factorize_schema_cleans_empty_defs() {
    let schema = v(json!({
        "$defs": {},
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    // Empty $defs should be removed
    assert!(result.get("$defs").is_none());
}

// --- factorize_schema with $defs present ---

#[test]
fn test_factorize_schema_preserves_defs() {
    let schema = v(json!({
        "$defs": {
            "City": {
                "properties": {"name": {"type": "string"}},
                "type": "object"
            }
        },
        "additionalProperties": false,
        "properties": {
            "entity": {
                "items": {"$ref": "#/$defs/City"},
                "title": "Entity",
                "type": "array"
            }
        },
        "required": ["entity"],
        "title": "Input",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    let defs = result.get("$defs").unwrap().as_object().unwrap();
    assert!(defs.contains_key("City"));
}

// --- concatenate_schema: $defs cleanup when empty ---

#[test]
fn test_concatenate_schema_removes_empty_defs() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"}
        },
        "required": ["foo"],
        "title": "Input",
        "type": "object"
    }));

    let result = concatenate_schema(&schema, &schema);
    // No $defs in inputs, result should have no $defs (or empty removed)
    if let Some(defs) = result.get("$defs") {
        // If present, should be empty which would be cleaned
        assert!(defs.as_object().map_or(true, |d| d.is_empty()));
    }
}

// --- out_mask_schema: no $defs at all ---

#[test]
fn test_out_mask_schema_no_defs() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["foo"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(!props.contains_key("foo"));
    assert!(props.contains_key("bar"));
}

// --- in_mask_schema: all keys removed, required cleaned ---

#[test]
fn test_in_mask_schema_all_removed() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["nonexistent"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
    // required should be removed entirely when no keys kept
    assert!(result.get("required").is_none());
}

// --- out_mask_schema: inline array items (no $defs), recursive ---

#[test]
fn test_out_mask_schema_inline_array_items_recursive() {
    // Array property with inline items schema (not using $defs/$ref)
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "things": {
                "title": "Things",
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "secret": {"title": "Secret", "type": "string"},
                        "name": {"title": "Name", "type": "string"}
                    },
                    "required": ["secret", "name"]
                }
            }
        },
        "required": ["things"],
        "title": "Input",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["secret"]), true);
    // "things" array should still be present
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("things"));
    // The items schema inside should have "secret" removed
    let things = props.get("things").unwrap();
    let items = things.get("items").unwrap();
    let item_props = items.get("properties").unwrap().as_object().unwrap();
    assert!(!item_props.contains_key("secret"));
    assert!(item_props.contains_key("name"));
    // Required inside items should be updated
    let item_req = items
        .get("required")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_owned()))
        .collect::<Vec<String>>();
    assert!(!item_req.contains(&"secret".to_owned()));
    assert!(item_req.contains(&"name".to_owned()));
}

// --- in_mask_schema: inline array items (no $defs), recursive ---

#[test]
fn test_in_mask_schema_inline_array_items_recursive() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "things": {
                "title": "Things",
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "name": {"title": "Name", "type": "string"},
                        "age": {"title": "Age", "type": "integer"},
                        "secret": {"title": "Secret", "type": "string"}
                    },
                    "required": ["name", "age", "secret"]
                }
            }
        },
        "required": ["things"],
        "title": "Input",
        "type": "object"
    }));

    // mask keeps "thing" (matches "things"), and "name" inside items
    let result = in_mask_schema(&schema, Some(&["thing", "name"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("things"));
    let things = props.get("things").unwrap();
    let items = things.get("items").unwrap();
    let item_props = items.get("properties").unwrap().as_object().unwrap();
    assert!(item_props.contains_key("name"));
    assert!(!item_props.contains_key("age"));
    assert!(!item_props.contains_key("secret"));
}

// --- in_mask_schema: required removed when becomes empty after retain ---

#[test]
fn test_in_mask_schema_required_removed_when_empty_after_retain() {
    // Schema where all required fields get masked out, testing required cleanup
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["foo", "bar"],
        "title": "Input",
        "type": "object"
    }));

    // Mask keeps nothing matching -> all removed, required should be cleaned
    let result = in_mask_schema(&schema, Some(&["baz"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
    // required should be removed (empty)
    assert!(result.get("required").is_none());
}

// --- concatenate_schema: $defs empty removal ---

#[test]
fn test_concatenate_schema_defs_removed_when_both_empty() {
    // Neither schema has $defs; the default empty $defs should be cleaned up
    let schema1 = v(json!({
        "properties": {"a": {"title": "A", "type": "string"}},
        "required": ["a"],
        "title": "S1",
        "type": "object"
    }));
    let schema2 = v(json!({
        "properties": {"b": {"title": "B", "type": "string"}},
        "required": ["b"],
        "title": "S2",
        "type": "object"
    }));
    let result = concatenate_schema(&schema1, &schema2);
    // $defs should be removed since both inputs had none
    assert!(result.get("$defs").is_none());
}

// --- factorize_schema: $defs empty removal ---

#[test]
fn test_factorize_schema_defs_removed_when_no_refs() {
    // Schema with no $defs, verify cleanup
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "foo_1": {"title": "Foo 1", "type": "string"}
        },
        "required": ["foo", "foo_1"],
        "title": "Input",
        "type": "object"
    }));
    let result = factorize_schema(&schema);
    assert!(result.get("$defs").is_none());
}

// --- prefix_schema / suffix_schema: schema with no properties key ---

#[test]
fn test_prefix_schema_no_properties_key() {
    let schema = v(json!({"title": "Empty", "type": "object"}));
    let result = prefix_schema(&schema, "pre");
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

#[test]
fn test_suffix_schema_no_properties_key() {
    let schema = v(json!({"title": "Empty", "type": "object"}));
    let result = suffix_schema(&schema, "suf");
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.is_empty());
}

// --- factorize_schema: array properties with different items (anyOf branch) ---

#[test]
fn test_factorize_schema_arrays_with_different_items() {
    // Two similar array properties with different items schemas
    let schema = v(json!({
        "additionalProperties": false,
        "$defs": {
            "TypeA": {"type": "object", "properties": {"a": {"type": "string"}}},
            "TypeB": {"type": "object", "properties": {"b": {"type": "string"}}}
        },
        "properties": {
            "item": {
                "type": "array",
                "items": {"$ref": "#/$defs/TypeA"},
                "description": "First list"
            },
            "item_1": {
                "type": "array",
                "items": {"$ref": "#/$defs/TypeB"}
            }
        },
        "required": ["item", "item_1"],
        "title": "Input",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    let props = result.get("properties").unwrap().as_object().unwrap();

    // Should be factorized into "items" (plural of "item")
    assert!(props.contains_key("items"));
    let items_prop = props.get("items").unwrap();

    // Should have anyOf in items since they have different items schemas
    let inner_items = items_prop.get("items").unwrap();
    assert!(inner_items.get("anyOf").is_some());
    let any_of = inner_items.get("anyOf").unwrap().as_array().unwrap();
    assert_eq!(any_of.len(), 2);

    // description should be removed
    assert!(items_prop.get("description").is_none());

    // $ref should be removed from items
    assert!(inner_items.get("$ref").is_none());

    // required should include "items"
    let required = result.get("required").unwrap().as_array().unwrap();
    assert!(required.iter().any(|v| v.as_str() == Some("items")));
}

// --- factorize_schema: single property (no similar) adds to required ---

#[test]
fn test_factorize_schema_single_prop_adds_required() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "name": {"title": "Name", "type": "string"}
        },
        "required": ["name"],
        "title": "Input",
        "type": "object"
    }));

    let result = factorize_schema(&schema);
    let required = result.get("required").unwrap().as_array().unwrap();
    assert!(required.iter().any(|v| v.as_str() == Some("name")));
}

// --- in_mask_schema: required becomes empty after retain ---

#[test]
fn test_in_mask_schema_required_empty_after_retain() {
    // Mask keeps "foo" but required only contains "bar"
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "foo": {"title": "Foo", "type": "string"},
            "bar": {"title": "Bar", "type": "string"}
        },
        "required": ["bar"],
        "title": "Input",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["foo"]), false);
    let props = result.get("properties").unwrap().as_object().unwrap();
    assert!(props.contains_key("foo"));
    assert!(!props.contains_key("bar"));
    // required should be removed since "bar" was the only required field
    // and it was not in the mask
    assert!(result.get("required").is_none());
}

// --- out_mask_schema: recursive with array items containing properties ---

#[test]
fn test_out_mask_schema_recursive_array_items_with_nested_props() {
    // Array property whose items have nested object properties
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "records": {
                "title": "Records",
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "secret": {"title": "Secret", "type": "string"},
                        "value": {"title": "Value", "type": "string"}
                    },
                    "required": ["secret", "value"]
                }
            },
            "name": {"title": "Name", "type": "string"}
        },
        "required": ["records", "name"],
        "title": "Input",
        "type": "object"
    }));

    let result = out_mask_schema(&schema, Some(&["secret"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();

    // "records" should still exist
    let records = props.get("records").unwrap();
    let items_inner = records.get("items").unwrap();
    let inner_props = items_inner.get("properties").unwrap().as_object().unwrap();
    // "secret" should be removed from array items
    assert!(!inner_props.contains_key("secret"));
    assert!(inner_props.contains_key("value"));
}

// --- in_mask_schema: recursive with array items containing properties ---

#[test]
fn test_in_mask_schema_recursive_array_items_with_nested_props() {
    let schema = v(json!({
        "additionalProperties": false,
        "properties": {
            "records": {
                "title": "Records",
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "keep_me": {"title": "Keep Me", "type": "string"},
                        "remove_me": {"title": "Remove Me", "type": "string"}
                    },
                    "required": ["keep_me", "remove_me"]
                }
            }
        },
        "required": ["records"],
        "title": "Input",
        "type": "object"
    }));

    let result = in_mask_schema(&schema, Some(&["record", "keep_me"]), true);
    let props = result.get("properties").unwrap().as_object().unwrap();
    let records = props.get("records").unwrap();
    let items_inner = records.get("items").unwrap();
    let inner_props = items_inner.get("properties").unwrap().as_object().unwrap();
    assert!(inner_props.contains_key("keep_me"));
    assert!(!inner_props.contains_key("remove_me"));
}
