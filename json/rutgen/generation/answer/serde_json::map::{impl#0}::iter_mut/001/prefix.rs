// Answer 0

#[test]
fn test_iter_mut_empty_map() {
    let mut map: Map<String, Value> = Map::new();
    let _iterator = map.iter_mut();
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let _iterator = map.iter_mut();
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map: Map<String, Value> = Map::with_capacity(10);
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    let _iterator = map.iter_mut();
}

#[test]
fn test_iter_mut_various_values() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("null_key".to_string(), Value::Null);
    map.insert("bool_key".to_string(), Value::Bool(false));
    map.insert("number_key".to_string(), Value::Number(100.into()));
    map.insert("string_key".to_string(), Value::String("a string".to_string()));
    map.insert("array_key".to_string(), Value::Array(vec![Value::String("elem".to_string())]));
    map.insert("object_key".to_string(), Value::Object(Map::new()));
    let _iterator = map.iter_mut();
}

#[test]
fn test_iter_mut_large_map() {
    let mut map: Map<String, Value> = Map::with_capacity(100);
    for i in 0..100 {
        map.insert(format!("key{}", i), Value::Number(i.into()));
    }
    let _iterator = map.iter_mut();
}

