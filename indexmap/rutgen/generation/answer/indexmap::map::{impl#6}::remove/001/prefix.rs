// Answer 0

#[test]
fn test_remove_existing_key() {
    struct TestKey(u32);
    struct TestValue(String);
    
    let mut map = IndexMap::new();
    map.insert(TestKey(1), TestValue("Value 1".to_string()));
    
    let result = map.remove(&TestKey(1));
}

#[test]
fn test_remove_non_existing_key() {
    struct TestKey(u32);
    struct TestValue(String);
    
    let mut map = IndexMap::new();
    map.insert(TestKey(1), TestValue("Value 1".to_string()));
    
    let result = map.remove(&TestKey(2));
}

#[test]
fn test_remove_empty_key() {
    struct TestKey(String);
    struct TestValue(String);
    
    let mut map = IndexMap::new();
    map.insert(TestKey("".to_string()), TestValue("Empty Key".to_string()));
    
    let result = map.remove(&TestKey("".to_string()));
}

#[test]
fn test_remove_large_key() {
    struct TestKey(String);
    struct TestValue(String);
    
    let mut map = IndexMap::new();
    map.insert(TestKey("This is a very large key that exceeds typical lengths".to_string()), TestValue("Large Value".to_string()));
    
    let result = map.remove(&TestKey("This is a very large key that exceeds typical lengths".to_string()));
}

#[test]
fn test_remove_numeric_key() {
    struct TestKey(i32);
    struct TestValue(String);
    
    let mut map = IndexMap::new();
    map.insert(TestKey(42), TestValue("The answer".to_string()));
    
    let result = map.remove(&TestKey(42));
}

