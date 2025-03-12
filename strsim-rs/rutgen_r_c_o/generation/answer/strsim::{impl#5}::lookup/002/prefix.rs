// Answer 0

#[test]
fn test_lookup_existing_key() {
    struct TestValue {
        data: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { data: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValue> {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: TestValue::default() }, // index 0
            GrowingHashmapMapElemChar { key: 1, value: TestValue { data: 42 } }, // index 1, existing key
            GrowingHashmapMapElemChar { key: 2, value: TestValue::default() }, // index 2
            GrowingHashmapMapElemChar { key: 3, value: TestValue::default() }, // index 3
        ]),
    };

    let key = 1;
    let result = hashmap.lookup(key);
}

#[test]
fn test_lookup_existing_key_another_case() {
    struct TestValue {
        data: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { data: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValue> {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: TestValue { data: 10 } }, // index 0
            GrowingHashmapMapElemChar { key: 1, value: TestValue::default() }, // index 1
            GrowingHashmapMapElemChar { key: 2, value: TestValue { data: 20 } }, // index 2, existing key
            GrowingHashmapMapElemChar { key: 3, value: TestValue::default() }, // index 3
        ]),
    };

    let key = 2;
    let result = hashmap.lookup(key);
}

