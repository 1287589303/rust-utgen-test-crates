// Answer 0

#[test]
fn test_get_index_of_existing_key() {
    struct TestKey;
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _other: &TestKey) -> bool {
            true
        }
    }

    let mut index_map = IndexMapCore::new();
    let hash = HashValue(1);
    let key = TestKey;

    index_map.push_entry(hash, key, "value");

    let result = index_map.get_index_of(hash, &key);
}

#[test]
fn test_get_index_of_non_existing_key() {
    struct TestKey;
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _other: &TestKey) -> bool {
            false
        }
    }

    let index_map = IndexMapCore::new();
    let hash = HashValue(1);
    let key = TestKey;

    let result = index_map.get_index_of(hash, &key);
}

#[test]
fn test_get_index_of_with_multiple_keys() {
    struct TestKey(usize);
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMapCore::new();
    let hash1 = HashValue(1);
    let key1 = TestKey(1);
    index_map.push_entry(hash1, key1.clone(), "value1");

    let hash2 = HashValue(2);
    let key2 = TestKey(2);
    index_map.push_entry(hash2, key2.clone(), "value2");

    let result1 = index_map.get_index_of(hash1, &key1);
    let result2 = index_map.get_index_of(hash2, &key2);
}

#[test]
fn test_get_index_of_boundary_case() {
    struct TestKey(usize);
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMapCore::<TestKey, &str>::new();
    let hash = HashValue(0);
    let key = TestKey(0);

    index_map.push_entry(hash, key, "value");

    let result = index_map.get_index_of(hash, &TestKey(0));
}

#[test]
fn test_get_index_of_empty_map() {
    struct TestKey;
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, _other: &TestKey) -> bool {
            false
        }
    }

    let index_map = IndexMapCore::<TestKey, &str>::new();
    let hash = HashValue(1);
    let key = TestKey;

    let result = index_map.get_index_of(hash, &key);
}

