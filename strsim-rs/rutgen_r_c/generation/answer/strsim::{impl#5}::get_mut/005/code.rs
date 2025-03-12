// Answer 0

#[test]
fn test_get_mut_initialization() {
    struct TestValueType {
        value: i32,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { value: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    hashmap.allocate(); // Ensure map is initialized

    let mut value = hashmap.get_mut(1); // Getting a mutable reference
    *value = TestValueType { value: 42 }; // Assigning a value

    assert_eq!(hashmap.get(1), TestValueType { value: 42 });
}

#[test]
fn test_get_mut_key_reused() {
    struct TestValueType {
        value: i32,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { value: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 2, // Set fill to 2 (to satisfy precondition for the test)
        mask: 7, // Initial mask for size 8
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 1, value: TestValueType { value: 0 } },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let mut value = hashmap.get_mut(1); // Accessing existing key
    *value = TestValueType { value: 100 }; // Updating its value

    assert_eq!(hashmap.get(1), TestValueType { value: 100 });
}

#[test]
fn test_get_mut_fill_limit() {
    struct TestValueType {
        value: i32,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { value: 0 }
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 3, // Near the limit before resizing
        mask: 7, // Initial mask for size 8
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 1, value: TestValueType { value: 5 } },
            GrowingHashmapMapElemChar { key: 2, value: TestValueType { value: 10 } },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let mut value = hashmap.get_mut(3); // Accessing new key
    *value = TestValueType { value: 20 }; // Assigning value should update and fall below fill limit for resize

    assert_eq!(hashmap.get(3), TestValueType { value: 20 });
}

