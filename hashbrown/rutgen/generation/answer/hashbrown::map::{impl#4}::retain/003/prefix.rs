// Answer 0

#[test]
fn test_retain_all_false() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder, Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);

    map.retain(|_key, _value| false);
}

#[test]
fn test_retain_predicate_never_true() {
    let mut map: HashMap<String, String> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder, Global);
    map.insert("a".to_string(), "apple".to_string());
    map.insert("b".to_string(), "banana".to_string());
    
    map.retain(|_key, _value| false);
}

#[test]
fn test_retain_empty_after_retain() {
    let mut map: HashMap<u32, u32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder, Global);
    map.insert(10, 100);
    
    map.retain(|_key, _value| false);
    assert_eq!(map.len(), 0);
}

