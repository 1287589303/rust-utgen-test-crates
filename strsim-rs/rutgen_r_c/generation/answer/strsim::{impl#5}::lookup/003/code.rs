// Answer 0

#[test]
fn test_lookup_returns_correct_index_when_key_is_not_found_in_the_hashmap() {
    struct TestValue {
        value: usize,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 1,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue { value: 10 } },
            GrowingHashmapMapElemChar { key: 2, value: TestValue { value: 20 } },
            GrowingHashmapMapElemChar { key: 3, value: TestValue::default() },
            GrowingHashmapMapElemChar { key: 4, value: TestValue::default() },
        ]),
    };

    let key = 1; // key in the map but not the default value
    let result = hashmap.lookup(key);
    assert_eq!(result, 3); // Expecting to find the default value at index 2 and return 3
}

#[test]
fn test_lookup_when_collision_resolved_on_default_value() {
    struct TestValue {
        value: usize,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 1,
        fill: 4,
        mask: 4,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestValue { value: 10 } },
            GrowingHashmapMapElemChar { key: 5, value: TestValue { value: 20 } },
            GrowingHashmapMapElemChar { key: 2, value: TestValue { value: 20 } },
            GrowingHashmapMapElemChar { key: 3, value: TestValue::default() },
            GrowingHashmapMapElemChar { key: 4, value: TestValue::default() },
        ]),
    };

    let key = 3; // key in the map with value set to default
    let result = hashmap.lookup(key);
    assert_eq!(result, 3); // Expecting to find the default value at index 3
}

