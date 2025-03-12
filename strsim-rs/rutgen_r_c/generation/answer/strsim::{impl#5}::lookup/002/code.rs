// Answer 0

#[test]
fn test_lookup_key_found() {
    struct TestValue {
        value: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: TestValue::default() }, // Default entry
            GrowingHashmapMapElemChar { key: 1, value: TestValue { value: 10 } }, // Entry with key 1
            GrowingHashmapMapElemChar { key: 2, value: TestValue { value: 20 } }, // Entry with key 2
            GrowingHashmapMapElemChar { key: 3, value: TestValue::default() }, // Default entry
        ]),
    };

    let key = 1;
    let expected_index = 1; // Since map[1] has key=1
    let actual_index = hashmap.lookup(key);

    assert_eq!(actual_index, expected_index);
}

#[test]
fn test_lookup_key_not_found_with_collision() {
    struct TestValue {
        value: i32,
    }

    impl Default for TestValue {
        fn default() -> Self {
            TestValue { value: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: TestValue::default() }, // Default entry
            GrowingHashmapMapElemChar { key: 1, value: TestValue { value: 10 } }, // Entry with key 1
            GrowingHashmapMapElemChar { key: 2, value: TestValue { value: 20 } }, // Entry with key 2
            GrowingHashmapMapElemChar { key: 3, value: TestValue { value: 30 } }, // Entry with key 3
        ]),
    };

    let key = 2;
    let expected_index = 2; // Since map[2] has key=2
    let actual_index = hashmap.lookup(key);

    assert_eq!(actual_index, expected_index);
}

