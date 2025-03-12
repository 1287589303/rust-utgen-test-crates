// Answer 0

#[test]
fn test_len_with_zero_capacity() {
    let map = Map::with_capacity(0);
    let length = map.len();
}

#[test]
fn test_len_after_insertion() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Null);
    let length_after_insertion = map.len();
    
    map.insert("key2".to_string(), Value::Bool(true));
    let length_after_insertion_two = map.len();
}

#[test]
fn test_len_with_duplicate_keys() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(Number::from(10)));
    map.insert("key1".to_string(), Value::Number(Number::from(20))); // overwrite
    let length = map.len();
}

#[test]
fn test_len_with_large_capacity() {
    let mut map = Map::with_capacity(1000);
    for i in 0..1000 {
        map.insert(format!("key{}", i), Value::Number(Number::from(i)));
    }
    let length = map.len();
}

#[test]
fn test_len_with_multiple_insertions() {
    let mut map = Map::<String, Value>::new();
    for i in 0..10 {
        map.insert(format!("key{}", i), Value::String(format!("value{}", i)));
        let curr_length = map.len();
    }
}

