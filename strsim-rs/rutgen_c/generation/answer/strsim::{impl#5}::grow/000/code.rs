// Answer 0

#[test]
fn test_grow_basic() {
    struct TestValueType {
        data: usize,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { data: 0 }
        }
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(), // index 0
            GrowingHashmapMapElemChar { key: 1, value: TestValueType { data: 10 } }, // index 1
            GrowingHashmapMapElemChar { key: 2, value: TestValueType { data: 20 } }, // index 2
            GrowingHashmapMapElemChar::default(), // index 3
        ]),
    };

    hashmap.grow(2);
    assert_eq!(hashmap.mask, 7);
    assert_eq!(hashmap.used, 2);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8);
}

#[test]
fn test_grow_increase_size() {
    struct TestValueType {
        data: usize,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { data: 0 }
        }
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(), // index 0
            GrowingHashmapMapElemChar { key: 1, value: TestValueType { data: 10 } }, // index 1
        ]),
    };

    hashmap.grow(3);
    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.used, 1);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 4);
}

#[test]
fn test_grow_empty_map() {
    struct TestValueType {
        data: usize,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { data: 0 }
        }
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 1]),
    };

    hashmap.grow(1);
    assert_eq!(hashmap.mask, 0);
    assert_eq!(hashmap.used, 0);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 2);
}

#[test]
fn test_grow_exceeding_current_capacity() {
    struct TestValueType {
        data: usize,
    }

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType { data: 0 }
        }
    }

    let mut hashmap: GrowingHashmapChar<TestValueType> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(), // index 0
            GrowingHashmapMapElemChar { key: 1, value: TestValueType { data: 10 } }, // index 1
        ]),
    };

    hashmap.grow(5);
    assert_eq!(hashmap.mask, 7);
    assert_eq!(hashmap.used, 1);
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8);
}

