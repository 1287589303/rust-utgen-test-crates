// Answer 0

#[test]
fn test_is_empty_on_new_map() {
    let map: Map<String, Value> = Map::new();
    map.is_empty();
}

#[test]
fn test_is_empty_on_map_with_one_element() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.is_empty();
}

#[test]
fn test_is_not_empty_on_map_with_multiple_elements() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(12.5)));
    let _ = map.is_empty();
}

#[test]
fn test_is_empty_on_cleared_map() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("test".to_string()));
    map.clear();
    map.is_empty();
}

