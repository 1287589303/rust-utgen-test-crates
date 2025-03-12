// Answer 0

#[test]
fn test_into_deserializer_single_element() {
    use std::collections::HashMap;
    
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key1".to_string(), 1);
    let deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_multiple_elements() {
    use std::collections::HashMap;
    
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    let deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_empty() {
    use std::collections::HashMap;
    
    let map: HashMap<String, i32> = HashMap::new();
    let deserializer = map.into_deserializer();
}

#[test]
fn test_into_deserializer_max_capacity() {
    use std::collections::HashMap;
    
    let mut map: HashMap<String, i32> = HashMap::with_capacity(10);
    for i in 0..10 {
        map.insert(format!("key{}", i), i);
    }
    let deserializer = map.into_deserializer();
}

