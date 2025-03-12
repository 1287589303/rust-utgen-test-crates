// Answer 0

#[test]
fn test_get_mut_existing_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let result = map.get_mut("key1");
}

#[test]
fn test_get_mut_non_existent_key() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let result = map.get_mut("key2");
}

#[test]
fn test_get_mut_empty_map() {
    let mut map = Map::new();
    let result = map.get_mut("key1");
}

#[test]
fn test_get_mut_with_capacity() {
    let mut map = Map::with_capacity(1);
    map.insert("key1".to_string(), Value::Number(Number::from(42)));
    let result = map.get_mut("key1");
}

#[test]
fn test_get_mut_edge_case_max_capacity() {
    let mut map = Map::with_capacity(10);
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::Number(Number::from(i)));
    }
    let result = map.get_mut("key9");
}

