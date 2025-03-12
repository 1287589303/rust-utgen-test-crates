// Answer 0

#[test]
fn test_serialize_with_non_empty_map() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(10)));
    
    let serializer = serde_json::Serializer::new(serde_json::de::from_str("{ }").unwrap());
    map.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_with_various_value_types() {
    let mut map = Map::new();
    map.insert("null_key".to_string(), Value::Null);
    map.insert("bool_key".to_string(), Value::Bool(false));
    map.insert("number_key".to_string(), Value::Number(serde_json::Number::from_f64(3.14).unwrap()));
    map.insert("string_key".to_string(), Value::String("example".to_string()));
    let array_value = Value::Array(vec![Value::Bool(true), Value::Number(serde_json::Number::from(5))]);
    map.insert("array_key".to_string(), array_value);
    
    let serializer = serde_json::Serializer::new(serde_json::de::from_str("{ }").unwrap());
    map.serialize(serializer).unwrap();
}

#[test]
fn test_serialize_with_empty_map_should_panics() {
    let map = Map::new();
    let serializer = serde_json::Serializer::new(serde_json::de::from_str("{ }").unwrap());
    let result = std::panic::catch_unwind(|| {
        map.serialize(serializer).unwrap();
    });
    assert!(result.is_err());
}

