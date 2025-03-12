// Answer 0

#[test]
fn test_clone_empty_map() {
    let original = Map { map: MapImpl::<String, Value>::new() };
    let cloned = original.clone();
}

#[test]
fn test_clone_map_with_null() {
    let mut original = Map { map: MapImpl::<String, Value>::new() };
    original.map.insert("key1".to_string(), Value::Null);
    let cloned = original.clone();
}

#[test]
fn test_clone_map_with_boolean() {
    let mut original = Map { map: MapImpl::<String, Value>::new() };
    original.map.insert("key2".to_string(), Value::Bool(true));
    let cloned = original.clone();
}

#[test]
fn test_clone_map_with_number() {
    let mut original = Map { map: MapImpl::<String, Value>::new() };
    original.map.insert("key3".to_string(), Value::Number(12.5.into()));
    let cloned = original.clone();
}

#[test]
fn test_clone_map_with_string() {
    let mut original = Map { map: MapImpl::<String, Value>::new() };
    original.map.insert("key4".to_string(), Value::String("a string".to_string()));
    let cloned = original.clone();
}

#[test]
fn test_clone_map_with_array() {
    let mut original = Map { map: MapImpl::<String, Value>::new() };
    original.map.insert("key5".to_string(), Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]));
    let cloned = original.clone();
}

#[test]
fn test_clone_map_with_object() {
    let mut inner_map = Map { map: MapImpl::<String, Value>::new() };
    inner_map.map.insert("inner_key".to_string(), Value::String("inner_value".to_string()));
    
    let mut original = Map { map: MapImpl::<String, Value>::new() };
    original.map.insert("key6".to_string(), Value::Object(inner_map));
    let cloned = original.clone();
}

