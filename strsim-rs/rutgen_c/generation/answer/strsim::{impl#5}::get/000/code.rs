// Answer 0

#[test]
fn test_get_when_map_is_empty_returns_default() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let result = hashmap.get(1);
    assert_eq!(result, TestValueType::default());
}

#[test]
fn test_get_when_key_exists_returns_value() {
    struct TestValueType {
        value: i32,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { value: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: TestValueType { value: 42 } }]),
    };
    
    let result = hashmap.get(1);
    assert_eq!(result.value, 42);
}

#[test]
fn test_get_when_key_does_not_exist_returns_default() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let hashmap = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 1, value: TestValueType }]),
    };

    let result = hashmap.get(2);
    assert_eq!(result, TestValueType::default());
}

