// Answer 0

#[test]
fn test_lookup_key_found() {
    struct TestValue(i32);

    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 1,
        fill: 2,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue(42) },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let result = hashmap.lookup(1);
    assert_eq!(result, 0);
}

#[test]
fn test_lookup_key_not_found() {
    struct TestValue(i32);

    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 2,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 2, value: TestValue(13) },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let result = hashmap.lookup(1);
    assert_eq!(result, 0);
}

#[test]
fn test_lookup_with_collision() {
    struct TestValue(i32);

    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 1,
        fill: 2,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 4, value: TestValue(7) },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 5, value: TestValue(14) },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let result = hashmap.lookup(5);
    assert_eq!(result, 2);
}

