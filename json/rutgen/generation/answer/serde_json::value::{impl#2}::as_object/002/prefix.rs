// Answer 0

#[test]
fn test_as_object_with_single_entry() {
    let map = serde_json::Map::from_iter(vec![(
        String::from("key1"),
        serde_json::Value::Bool(true),
    )]);
    let value = serde_json::Value::Object(map);
    let _result = value.as_object();
}

#[test]
fn test_as_object_with_multiple_entries() {
    let map = serde_json::Map::from_iter(vec![
        (String::from("key1"), serde_json::Value::Number(serde_json::Number::from(1))),
        (String::from("key2"), serde_json::Value::String(String::from("value"))),
    ]);
    let value = serde_json::Value::Object(map);
    let _result = value.as_object();
}

#[test]
fn test_as_object_with_empty_map() {
    let map = serde_json::Map::new();
    let value = serde_json::Value::Object(map);
    let _result = value.as_object();
}

