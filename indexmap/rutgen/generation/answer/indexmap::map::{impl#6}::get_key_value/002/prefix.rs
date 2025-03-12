// Answer 0

#[test]
fn test_get_key_value_existing_entry() {
    struct TestKey(u32);
    struct TestValue(String);
    
    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMap::<TestKey, TestValue, ()>::new();
    index_map.insert(TestKey(1), TestValue("Value1".to_string()));
    index_map.insert(TestKey(2), TestValue("Value2".to_string()));
    
    let result = index_map.get_key_value(&TestKey(1));
    let _ = result; // use the result to ensure it compiles
}

#[test]
fn test_get_key_value_with_largest_key() {
    struct TestKey(u32);
    struct TestValue(String);
    
    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMap::<TestKey, TestValue, ()>::new();
    index_map.insert(TestKey(0), TestValue("Value0".to_string()));
    index_map.insert(TestKey(u32::MAX), TestValue("MaxValue".to_string()));
    
    let result = index_map.get_key_value(&TestKey(u32::MAX));
    let _ = result; // use the result to ensure it compiles
}

#[test]
fn test_get_key_value_with_smallest_key() {
    struct TestKey(i32);
    struct TestValue(String);
    
    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut index_map = IndexMap::<TestKey, TestValue, ()>::new();
    index_map.insert(TestKey(-1), TestValue("NegativeValue".to_string()));
    index_map.insert(TestKey(0), TestValue("ZeroValue".to_string()));

    let result = index_map.get_key_value(&TestKey(-1));
    let _ = result; // use the result to ensure it compiles
}

