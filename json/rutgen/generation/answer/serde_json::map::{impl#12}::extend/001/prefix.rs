// Answer 0

#[test]
fn test_extend_empty() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let empty_iter: Vec<(String, Value)> = Vec::new();
    map.extend(empty_iter);
}

#[test]
fn test_extend_single_entry() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let single_entry = vec![(String::from("key1"), Value::Bool(true))];
    map.extend(single_entry);
}

#[test]
fn test_extend_multiple_entries() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let multiple_entries = vec![
        (String::from("key1"), Value::Number(12.into())),
        (String::from("key2"), Value::Null),
        (String::from("key3"), Value::String(String::from("text"))),
    ];
    map.extend(multiple_entries);
}

#[test]
fn test_extend_with_various_value_types() {
    let mut map = Map {
        map: MapImpl::new(),
    };
    let diverse_entries = vec![
        (String::from("null_key"), Value::Null),
        (String::from("bool_key"), Value::Bool(false)),
        (String::from("number_key"), Value::Number(10.5.into())),
        (String::from("string_key"), Value::String(String::from("example"))),
        (String::from("array_key"), Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())])),
        (String::from("object_key"), Value::Object(Map { map: MapImpl::new() })),
    ];
    map.extend(diverse_entries);
}

