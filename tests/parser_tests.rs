use json_parser::{JsonValue, get_field, parse_json, validate_json};

#[test]
fn test_invalid_missing_brace() {
    assert!(!validate_json("{\"a\": 1"));
}

#[test]
fn test_invalid_missing_colon() {
    assert!(!validate_json("{\"a\" 1}"));
}

#[test]
fn test_invalid_trailing_comma_object() {
    assert!(!validate_json("{\"a\":1,}"));
}

#[test]
fn test_invalid_trailing_comma_array() {
    assert!(!validate_json("[1,2,]"));
}

#[test]
fn test_invalid_double_comma() {
    assert!(!validate_json("{\"a\":1,,\"b\":2}"));
}

#[test]
fn test_invalid_bareword() {
    assert!(!validate_json("{a: 1}"));
}

#[test]
fn test_string_with_unicode_escape() {
    let input = "\"abc \\u1234 def\"";
    let val = parse_json(input).unwrap();

    if let JsonValue::String(s) = val {
        assert!(s.contains("\\u1234"));
    }
}

#[test]
fn test_number_big() {
    let val = parse_json("123456789012345").unwrap();
    if let JsonValue::Number(n) = val {
        assert_eq!(n, 123456789012345.0);
    }
}

#[test]
fn test_invalid_fraction() {
    assert!(!validate_json("1."));
}

#[test]
fn test_array_single_value() {
    let val = parse_json("[42]").unwrap();
    if let JsonValue::Array(arr) = val {
        assert_eq!(arr.len(), 1);
    }
}

#[test]
fn test_array_nested_mixed() {
    let val = parse_json("[1, [2, [3]], 4]").unwrap();
    if let JsonValue::Array(arr) = val {
        assert_eq!(arr.len(), 3);
    }
}

#[test]
fn test_array_with_objects() {
    let val = parse_json("[{\"x\":1}, {\"y\":2}]").unwrap();
    if let JsonValue::Array(arr) = val {
        assert_eq!(arr.len(), 2);
    }
}

#[test]
fn test_object_deep_nesting() {
    let val = parse_json("{\"a\":{\"b\":{\"c\":{\"d\":1}}}}").unwrap();

    if let JsonValue::Object(a) = val {
        if let JsonValue::Object(b) = a.get("a").unwrap() {
            if let JsonValue::Object(c) = b.get("b").unwrap() {
                if let JsonValue::Object(d) = c.get("c").unwrap() {
                    if let JsonValue::Number(num) = d.get("d").unwrap() {
                        assert_eq!(*num, 1.0);
                    }
                }
            }
        }
    }
}

#[test]
fn test_object_key_retrieval_multiple_types() {
    let json = r#"{
        "num": 10,
        "str": "abc",
        "bool": true,
        "null": null,
        "arr": [1,2],
        "obj": {"x":1}
    }"#;

    let root = parse_json(json).unwrap();

    assert!(matches!(
        get_field(&root, "num"),
        Some(JsonValue::Number(_))
    ));
    assert!(matches!(
        get_field(&root, "str"),
        Some(JsonValue::String(_))
    ));
    assert!(matches!(get_field(&root, "bool"), Some(JsonValue::Bool(_))));
    assert!(matches!(get_field(&root, "null"), Some(JsonValue::Null)));
    assert!(matches!(get_field(&root, "arr"), Some(JsonValue::Array(_))));
    assert!(matches!(
        get_field(&root, "obj"),
        Some(JsonValue::Object(_))
    ));
}

#[test]
fn test_root_value_string() {
    let val = parse_json("\"rty\"").unwrap();
    if let JsonValue::String(s) = val {
        assert_eq!(s, "rty");
    }
}

#[test]
fn test_root_value_number() {
    let val = parse_json("53").unwrap();
    if let JsonValue::Number(n) = val {
        assert_eq!(n, 53.0);
    }
}

#[test]
fn test_root_value_array() {
    let val = parse_json("[1,2]").unwrap();
    if let JsonValue::Array(arr) = val {
        assert_eq!(arr.len(), 2);
    }
}

#[test]
fn test_root_value_object() {
    let val = parse_json("{\"a\":1}").unwrap();
    if let JsonValue::Object(map) = val {
        assert!(map.contains_key("a"));
    }
}

#[test]
fn test_nested_array_of_objects() {
    let json = "[{\"a\":1},{\"b\":2},{\"c\":3}]";
    let val = parse_json(json).unwrap();

    if let JsonValue::Array(arr) = val {
        assert_eq!(arr.len(), 3);
    }
}

#[test]
fn test_invalid_json_extra_characters() {
    assert!(!validate_json("{\"a\":1} x"));
}

#[test]
fn test_invalid_json_missing_comma_between_pairs() {
    assert!(!validate_json("{\"a\":1 \"b\":2}"));
}
