// Answer 0

#[test]
fn test_insert_full_empty() {
    struct TestKey(usize);
    struct TestValue(String);

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash = HashValue(0);
    let key = TestKey(0);
    let value = TestValue("value".to_string());

    index_map.insert_full(hash, key, value);
}

#[test]
fn test_insert_full_single_entry() {
    struct TestKey(usize);
    struct TestValue(String);

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash = HashValue(0);
    let key = TestKey(0);
    let value = TestValue("value".to_string());

    index_map.insert_full(hash, key, value);

    let hash2 = HashValue(1);
    let key2 = TestKey(1);
    let value2 = TestValue("value2".to_string());

    index_map.insert_full(hash2, key2, value2);
}

#[test]
fn test_insert_full_multiple_entries() {
    struct TestKey(usize);
    struct TestValue(String);

    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    for i in 0..10 {
        let hash = HashValue(i as usize);
        let key = TestKey(i);
        let value = TestValue(format!("value{}", i));
        
        index_map.insert_full(hash, key, value);
    }
}

#[test]
fn test_insert_full_duplicate_key() {
    struct TestKey(usize);
    struct TestValue(String);
    
    let mut index_map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash = HashValue(0);
    let key = TestKey(0);
    let value = TestValue("value".to_string());

    index_map.insert_full(hash, key, value);

    let new_value = TestValue("new_value".to_string());
    let result = index_map.insert_full(hash, key, new_value);

    assert_eq!(result.0, 0);
    assert!(result.1.is_some());
}

