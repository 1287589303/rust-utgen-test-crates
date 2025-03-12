// Answer 0

#[test]
fn test_insert_full_vacant_entry_case() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let hash = HashValue(5);
    let key = TestKey(1);
    let value = TestValue(1);

    map.insert_full(hash, key, value);
}

#[test]
fn test_insert_full_vacant_entry_case_with_different_key() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let hash = HashValue(5);
    let key1 = TestKey(1);
    let value1 = TestValue(1);
    let key2 = TestKey(2);
    let value2 = TestValue(2);

    map.insert_full(hash, key1, value1);
    map.insert_full(hash, key2, value2);
}

#[test]
fn test_insert_full_with_higher_hash() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let hash = HashValue(10);
    let key = TestKey(2);
    let value = TestValue(2);

    map.insert_full(hash, key, value);
}

#[test]
fn test_insert_full_with_different_value() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let hash = HashValue(3);
    let key = TestKey(3);
    let value1 = TestValue(3);
    let value2 = TestValue(4);

    map.insert_full(hash, key, value1);
    map.insert_full(hash, key, value2);
}

#[test]
fn test_insert_full_boundary_hash() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let hash = HashValue(0);
    let key = TestKey(1);
    let value = TestValue(1);

    map.insert_full(hash, key, value);
}

#[test]
fn test_insert_full_boundary_key_value() {
    struct TestKey(usize);
    struct TestValue(usize);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    
    let hash = HashValue(99);
    let key = TestKey(10);
    let value = TestValue(10);

    map.insert_full(hash, key, value);
}

