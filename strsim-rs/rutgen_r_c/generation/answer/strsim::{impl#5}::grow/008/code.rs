// Answer 0

#[test]
fn test_grow_min_used_greater_than_initial_size() {
    struct TestValue {
        data: i32,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3, // Initially small for this example
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(5); // Ensure min_used > initial size results in growth
    assert_eq!(hashmap.mask, 7); // Expects a growth -- mask should be 7 (new_size - 1)
}

#[test]
fn test_grow_with_no_elements_in_old_map() {
    struct TestValue {
        data: i32,
    }

    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3, // Initial size
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    hashmap.grow(1); // Min_used is less than current elements in old_map (which are none)
    assert_eq!(hashmap.used, 0); // Should still have 0 used after growing with no elements
    assert_eq!(hashmap.fill, 0); // Fill should also remain 0
}

