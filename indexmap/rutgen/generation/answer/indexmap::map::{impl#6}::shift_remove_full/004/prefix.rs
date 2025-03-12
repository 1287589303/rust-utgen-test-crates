// Answer 0

#[test]
fn test_shift_remove_full_empty_map() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let key = TestKey;

    let result = map.shift_remove_full(&key);
}

#[test]
fn test_shift_remove_full_single_entry_not_matching() {
    struct TestKey;
    struct TestValue;

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::new();
    let non_matching_key = TestKey;
    let key = TestKey;
    map.insert(key, TestValue);

    let result = map.shift_remove_full(&non_matching_key);
}

