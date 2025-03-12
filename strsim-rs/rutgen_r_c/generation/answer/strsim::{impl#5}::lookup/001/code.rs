// Answer 0

#[test]
fn test_lookup_with_default_value() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar {
                key: 0,
                value: TestValueType::default(),
            },
            GrowingHashmapMapElemChar {
                key: 1,
                value: TestValueType::default(),
            },
        ]),
    };

    let result = hashmap.lookup(0);
    assert_eq!(result, 0);
}

#[test]
fn test_lookup_with_existing_key() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 1,
        fill: 2,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar {
                key: 0,
                value: TestValueType::default(),
            },
            GrowingHashmapMapElemChar {
                key: 1,
                value: TestValueType,
            },
        ]),
    };

    let result = hashmap.lookup(1);
    assert_eq!(result, 1);
}

#[test]
fn test_lookup_with_non_existing_key_and_default_value() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 1,
        fill: 2,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar {
                key: 0,
                value: TestValueType,
            },
            GrowingHashmapMapElemChar {
                key: 1,
                value: TestValueType::default(),
            },
        ]),
    };

    let result = hashmap.lookup(0);
    assert_eq!(result, 0);
}

