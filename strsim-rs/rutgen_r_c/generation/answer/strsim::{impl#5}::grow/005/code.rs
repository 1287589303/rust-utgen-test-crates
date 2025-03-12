// Answer 0

#[test]
fn test_grow_case_new_size_greater_than_min_used() {
    struct TestValue(u32);
    
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValue> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue(10) },
            GrowingHashmapMapElemChar { key: 2, value: TestValue(20) },
            GrowingHashmapMapElemChar { key: 3, value: TestValue(30) },
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(5);
    
    assert_eq!(hashmap.mask, 7); // new_size is 8, mask should be 7
    assert_eq!(hashmap.used, hashmap.fill); // used should stay as fill
}

#[test]
fn test_grow_case_elem_in_old_map() {
    struct TestValue(u32);
    
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValue> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue(10) },
            GrowingHashmapMapElemChar { key: 2, value: TestValue(20) },
            GrowingHashmapMapElemChar { key: 3, value: TestValue(30) },
            GrowingHashmapMapElemChar { key: 4, value: TestValue(0) }, // Should not affect the behavior.
        ]),
    };

    hashmap.grow(5);

    assert!(hashmap.map.as_ref().unwrap()[0].value != Default::default());
}

#[test]
fn test_grow_case_elem_value_not_default() {
    struct TestValue(u32);
    
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValue> {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue(10) },
            GrowingHashmapMapElemChar { key: 2, value: TestValue(20) },
            GrowingHashmapMapElemChar { key: 3, value: TestValue(0) }, // Should be default
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(5);

    assert!(hashmap.map.as_ref().unwrap()[0].value != Default::default());
    assert!(hashmap.map.as_ref().unwrap()[1].value != Default::default());
}

#[test]
fn test_grow_case_used_not_zero() {
    struct TestValue(u32);
    
    impl Default for TestValue {
        fn default() -> Self {
            TestValue(0)
        }
    }

    let mut hashmap = GrowingHashmapChar::<TestValue> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue(10) },
            GrowingHashmapMapElemChar { key: 2, value: TestValue(20) },
            GrowingHashmapMapElemChar { key: 3, value: TestValue(30) },
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(5);

    assert!(hashmap.used != 0); // used should still be greater than 0
}

