// Answer 0

#[test]
fn test_values_empty_map() {
    let map = Map::<String, Value>::new();
    let values = map.values();
}

#[test]
fn test_values_with_null() {
    let mut map = Map::<String, Value>::new();
    map.insert("key1".to_string(), Value::Null);
    let values = map.values();
}

#[test]
fn test_values_with_bool() {
    let mut map = Map::<String, Value>::new();
    map.insert("key2".to_string(), Value::Bool(true));
    let values = map.values();
}

#[test]
fn test_values_with_number() {
    let mut map = Map::<String, Value>::new();
    map.insert("key3".to_string(), Value::Number(serde_json::Number::from(42)));
    let values = map.values();
}

#[test]
fn test_values_with_string() {
    let mut map = Map::<String, Value>::new();
    map.insert("key4".to_string(), Value::String("a string".to_string()));
    let values = map.values();
}

#[test]
fn test_values_with_array() {
    let mut map = Map::<String, Value>::new();
    map.insert("key5".to_string(), Value::Array(vec![Value::String("element".to_string())]));
    let values = map.values();
}

#[test]
fn test_values_with_object() {
    let mut map = Map::<String, Value>::new();
    let mut object_map = Map::<String, Value>::new();
    object_map.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    map.insert("key6".to_string(), Value::Object(object_map));
    let values = map.values();
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_values_with_preserve_order() {
    let mut map = Map::<String, Value>::with_capacity(5);
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(3.14)));
    let values = map.values();
}

