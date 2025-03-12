// Answer 0

#[test]
fn test_clear_non_empty_map() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(3.14)));
    map.clear();
}

#[test]
fn test_clear_empty_map() {
    let mut map = Map::new();
    map.clear();
}

#[test]
fn test_clear_map_with_one_element() {
    let mut map = Map::new();
    map.insert("single_key".to_string(), Value::String("single_value".to_string()));
    map.clear();
}

#[test]
fn test_clear_map_with_multiple_elements() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(Number::from(1)));
    map.insert("key2".to_string(), Value::Null);
    map.insert("key3".to_string(), Value::Array(vec![Value::Bool(false)]));
    map.clear();
}

