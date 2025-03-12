// Answer 0

#[test]
fn test_new_empty_map() {
    let map = Map::<String, Value>::new();
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_map_with_null_value() {
    let mut map = Map::<String, Value>::new();
    map.insert("key1".to_string(), Value::Null);
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_map_with_bool_value() {
    let mut map = Map::<String, Value>::new();
    map.insert("key2".to_string(), Value::Bool(true));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_map_with_number_value() {
    let mut map = Map::<String, Value>::new();
    map.insert("key3".to_string(), Value::Number(Number::from(5)));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_map_with_string_value() {
    let mut map = Map::<String, Value>::new();
    map.insert("key4".to_string(), Value::String("example".to_string()));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_map_with_array_value() {
    let mut map = Map::<String, Value>::new();
    map.insert("key5".to_string(), Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]));
    let deserializer = MapDeserializer::new(map);
}

#[test]
fn test_new_map_with_nested_object_value() {
    let mut inner_map = Map::<String, Value>::new();
    inner_map.insert("inner_key".to_string(), Value::String("inner_value".to_string()));

    let mut outer_map = Map::<String, Value>::new();
    outer_map.insert("key6".to_string(), Value::Object(inner_map));
    let deserializer = MapDeserializer::new(outer_map);
}

