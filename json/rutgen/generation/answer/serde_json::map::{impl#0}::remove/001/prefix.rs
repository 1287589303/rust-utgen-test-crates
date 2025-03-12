// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let _ = map.remove("key1");
}

#[test]
fn test_remove_borrowed_key() {
    let mut map = Map::new();
    map.insert("key2".to_string(), Value::Number(42.into()));
    let key = "key2";
    let _ = map.remove(&key);
}

#[test]
fn test_remove_non_existent_key() {
    let mut map = Map::new();
    let result = map.remove("non_existent_key");
}

#[test]
fn test_remove_empty_key() {
    let mut map = Map::new();
    map.insert("".to_string(), Value::String("empty".to_string()));
    let _ = map.remove("");
}

#[test]
fn test_remove_max_key_length() {
    let mut map = Map::new();
    let max_key = "a".repeat(usize::MAX);
    map.insert(max_key.clone(), Value::Null);
    let _ = map.remove(max_key.as_str());
}

#[test]
fn test_remove_value_null() {
    let mut map = Map::new();
    map.insert("key_null".to_string(), Value::Null);
    let _ = map.remove("key_null");
}

#[test]
fn test_remove_value_bool() {
    let mut map = Map::new();
    map.insert("key_bool".to_string(), Value::Bool(false));
    let _ = map.remove("key_bool");
}

#[test]
fn test_remove_value_number() {
    let mut map = Map::new();
    map.insert("key_number".to_string(), Value::Number(99.into()));
    let _ = map.remove("key_number");
}

#[test]
fn test_remove_value_string() {
    let mut map = Map::new();
    map.insert("key_string".to_string(), Value::String("string_value".to_string()));
    let _ = map.remove("key_string");
}

#[test]
fn test_remove_value_array() {
    let mut map = Map::new();
    map.insert("key_array".to_string(), Value::Array(vec![Value::Bool(true), Value::Number(1.into())]));
    let _ = map.remove("key_array");
}

#[test]
fn test_remove_value_object() {
    let mut map = Map::new();
    let mut obj = Map::new();
    obj.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    map.insert("key_object".to_string(), Value::Object(obj));
    let _ = map.remove("key_object");
}

