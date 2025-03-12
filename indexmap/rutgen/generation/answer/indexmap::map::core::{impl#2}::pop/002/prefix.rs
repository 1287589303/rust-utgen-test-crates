// Answer 0

#[test]
fn test_pop_with_single_entry() {
    #[derive(Debug, PartialEq)]
    struct TestKey(usize);
    
    #[derive(Debug, PartialEq)]
    struct TestValue(String);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash = HashValue(0);
    let key = TestKey(1);
    let value = TestValue("one".to_string());
    
    map.entries.push(Bucket { hash, key, value });
    
    let result = map.pop();
}

#[test]
fn test_pop_with_multiple_entries() {
    #[derive(Debug, PartialEq)]
    struct TestKey(usize);
    
    #[derive(Debug, PartialEq)]
    struct TestValue(String);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash1 = HashValue(1);
    let key1 = TestKey(1);
    let value1 = TestValue("one".to_string());
    
    let hash2 = HashValue(2);
    let key2 = TestKey(2);
    let value2 = TestValue("two".to_string());
    
    map.entries.push(Bucket { hash: hash1, key: key1, value: value1 });
    map.entries.push(Bucket { hash: hash2, key: key2, value: value2 });
    
    let result = map.pop();
}

#[test]
fn test_pop_removes_last_entry() {
    #[derive(Debug, PartialEq)]
    struct TestKey(usize);
    
    #[derive(Debug, PartialEq)]
    struct TestValue(String);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash = HashValue(0);
    let key = TestKey(1);
    let value = TestValue("one".to_string());
    
    map.entries.push(Bucket { hash, key, value });

    let before_pop_len = map.len();
    let _result = map.pop();
    
    let after_pop_len = map.len();
}

#[test]
fn test_pop_with_hash_calculations() {
    #[derive(Debug, PartialEq)]
    struct TestKey(usize);
    
    #[derive(Debug, PartialEq)]
    struct TestValue(String);
    
    let mut map: IndexMapCore<TestKey, TestValue> = IndexMapCore::new();
    let hash = HashValue(3);
    let key = TestKey(3);
    let value = TestValue("three".to_string());
    
    map.entries.push(Bucket { hash, key, value });
    
    let result = map.pop();
}

