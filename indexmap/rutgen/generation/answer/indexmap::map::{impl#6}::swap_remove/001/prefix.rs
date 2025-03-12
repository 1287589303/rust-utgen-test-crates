// Answer 0

#[test]
fn test_swap_remove_existing_key() {
    struct TestKey(i32);
    struct TestValue(i32);

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    map.insert(TestKey(1), TestValue(10));
    map.insert(TestKey(2), TestValue(20));
    
    let result = map.swap_remove(&TestKey(1));
}

#[test]
fn test_swap_remove_non_existing_key() {
    struct TestKey(i32);
    struct TestValue(i32);

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    map.insert(TestKey(1), TestValue(10));
    map.insert(TestKey(2), TestValue(20));
    
    let result = map.swap_remove(&TestKey(3));
}

#[test]
fn test_swap_remove_last_element() {
    struct TestKey(i32);
    struct TestValue(i32);

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    map.insert(TestKey(1), TestValue(10));
    
    let result = map.swap_remove(&TestKey(1));
}

#[test]
fn test_swap_remove_empty_map() {
    struct TestKey(i32);
    struct TestValue(i32);

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    
    let result = map.swap_remove(&TestKey(1));
}

#[test]
fn test_swap_remove_large_key() {
    struct TestKey(i64);
    struct TestValue(i32);

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    map.insert(TestKey(i64::MAX), TestValue(100));
    
    let result = map.swap_remove(&TestKey(i64::MAX));
}

