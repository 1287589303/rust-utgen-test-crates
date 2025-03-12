// Answer 0

#[test]
fn test_swap_remove_full_single_entry() {
    struct TestKey;
    struct TestValue;

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _: &TestKey) -> bool {
            true
        }
    }

    let mut map: IndexMap<TestKey, TestValue, RandomState> = IndexMap::with_capacity(1);
    let test_key = TestKey;
    let test_value = TestValue;

    let hash_value = map.hash(&test_key);
    map.core.push_entry(hash_value, test_key, test_value);

    let result = map.swap_remove_full(&test_key);
}

