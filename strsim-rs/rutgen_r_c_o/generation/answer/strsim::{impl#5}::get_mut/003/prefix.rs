// Answer 0

#[test]
fn test_get_mut_when_map_is_none_and_value_is_non_default() {
    struct TestValue(u32);
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let key: u32 = 42;
    let value = hashmap.get_mut(key);
    *value = TestValue(1);
}

#[test]
fn test_get_mut_on_empty_hashmap() {
    struct TestValue(u32);
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    let key: u32 = 100;
    let value = hashmap.get_mut(key);
    *value = TestValue(5);
}

#[test]
fn test_get_mut_on_growth_trigger() {
    struct TestValue(u32);
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 6,
        fill: 6,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 1, value: TestValue(1) },
            GrowingHashmapMapElemChar { key: 2, value: TestValue(2) },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 4, value: TestValue(4) },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let key: u32 = 99;
    let value = hashmap.get_mut(key);
    *value = TestValue(10);
}

