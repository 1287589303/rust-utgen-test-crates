// Answer 0

#[test]
fn test_from_iter_with_non_empty_pairs() {
    let v: Vec<(&str, Value)> = vec![
        ("key1", Value::Bool(true)),
        ("key2", Value::Number(Number { n: 42 })),
        ("key3", Value::String("value".to_string())),
    ];
    let _x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_with_empty_string_key() {
    let v: Vec<(&str, Value)> = vec![
        ("", Value::Bool(false)),
        ("key2", Value::Number(Number { n: 0 })),
    ];
    let _x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_with_extreme_numeric_values() {
    let v: Vec<(&str, Value)> = vec![
        ("max", Value::Number(Number { n: std::i64::MAX })),
        ("min", Value::Number(Number { n: std::i64::MIN })),
    ];
    let _x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_with_various_value_types() {
    let v: Vec<(&str, Value)> = vec![
        ("bool", Value::Bool(true)),
        ("number", Value::Number(Number { n: 3.14 })),
        ("string", Value::String("text".to_owned())),
        ("array", Value::Array(vec![Value::Bool(false), Value::Number(Number { n: 5 })])),
        ("object", Value::Object(Map::new())),
    ];
    let _x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_with_empty_iterator() {
    let v: Vec<(&str, Value)> = vec![];
    let _x: Value = v.into_iter().collect();
}

