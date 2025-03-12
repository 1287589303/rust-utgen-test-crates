// Answer 0

#[test]
fn test_iter_empty_map() {
    let map = Map::new();
    let iter = map.iter();
}

#[test]
fn test_iter_single_pair() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let iter = map.iter();
}

#[test]
fn test_iter_multiple_pairs() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    map.insert("key2".to_string(), Value::Number(Number::from(42)));
    let iter = map.iter();
}

#[test]
fn test_iter_mixed_types() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    map.insert("key2".to_string(), Value::Bool(false));
    map.insert("key3".to_string(), Value::Number(Number::from(3.14)));
    map.insert("key4".to_string(), Value::String("example".to_string()));
    map.insert("key5".to_string(), Value::Array(vec![Value::String("item".to_string())]));
    map.insert("key6".to_string(), Value::Object(Map::new()));
    let iter = map.iter();
}

#[test]
fn test_iter_with_capacity() {
    let mut map = Map::with_capacity(10);
    map.insert("key1".to_string(), Value::Number(Number::from(1)));
    let iter = map.iter();
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_iter_with_preserve_order() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let iter = map.iter();
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_iter_ordered_multiple_pairs() {
    let mut map = Map::new();
    map.insert("key2".to_string(), Value::Number(Number::from(24)));
    map.insert("key1".to_string(), Value::String("first".to_string()));
    let iter = map.iter();
}

#[test]
fn test_iter_duplicate_keys() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("first".to_string()));
    map.insert("key1".to_string(), Value::String("second".to_string())); // This should overwrite the first
    let iter = map.iter();
}

