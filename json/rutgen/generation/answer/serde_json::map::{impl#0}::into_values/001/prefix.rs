// Answer 0

#[test]
fn test_into_values_empty_map() {
    let map: Map<String, Value> = Map::new();
    let _ = map.into_values();
}

#[test]
fn test_into_values_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_owned(), Value::Bool(true));
    let _ = map.into_values();
}

#[test]
fn test_into_values_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_owned(), Value::Number(Number::from(42)));
    map.insert("key2".to_owned(), Value::String("value".to_owned()));
    map.insert("key3".to_owned(), Value::Array(vec![Value::Null]));
    let _ = map.into_values();
}

#[test]
fn test_into_values_different_value_types() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_owned(), Value::Null);
    map.insert("key2".to_owned(), Value::Bool(false));
    map.insert("key3".to_owned(), Value::Number(Number::from(12.34)));
    map.insert("key4".to_owned(), Value::String("string value".to_owned()));
    map.insert("key5".to_owned(), Value::Array(vec![Value::Bool(true), Value::Number(Number::from(1))]));
    map.insert("key6".to_owned(), Value::Object(Map::new()));
    let _ = map.into_values();
}

