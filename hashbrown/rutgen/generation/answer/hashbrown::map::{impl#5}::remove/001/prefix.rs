// Answer 0

#[test]
fn test_remove_existing_key() {
    struct Key(i32);
    struct Value(&'static str);
    
    let mut map = HashMap::new();
    map.insert(Key(1), Value("a"));
    
    let result = map.remove(&Key(1)); // Q is Key
}

#[test]
fn test_remove_existing_key_multiple_entries() {
    struct Key(i32);
    struct Value(&'static str);
    
    let mut map = HashMap::new();
    map.insert(Key(1), Value("a"));
    map.insert(Key(2), Value("b"));
    
    let result = map.remove(&Key(1)); // Q is Key
}

#[test]
fn test_remove_existing_key_value_retained() {
    struct Key(i32);
    struct Value(&'static str);
    
    let mut map = HashMap::new();
    map.insert(Key(3), Value("c"));
    
    let result = map.remove(&Key(3)); // Q is Key
}

#[test]
fn test_remove_key_from_nonempty_map() {
    struct Key(i32);
    struct Value(&'static str);
    
    let mut map = HashMap::new();
    map.insert(Key(4), Value("d"));
    map.insert(Key(5), Value("e"));
    
    let result = map.remove(&Key(4)); // Q is Key
}

