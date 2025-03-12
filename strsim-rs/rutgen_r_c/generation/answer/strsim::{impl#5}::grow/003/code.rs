// Answer 0

#[test]
fn test_grow_with_new_size_equal_to_min_used() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(4); // new_size will be 4, which is equal to min_used
    assert_eq!(hashmap.mask, 3); // Expect mask to be 3 after grow
    assert_eq!(hashmap.used, 3);  // Used should remain the same
}

#[test]
fn test_grow_with_new_size_greater_than_min_used() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(2); // new_size will be 4 and should be greater than min_used
    assert_eq!(hashmap.mask, 3); // Expect mask to remain the same
    assert_eq!(hashmap.used, 3);  // Used should remain the same
}

#[test]
fn test_grow_when_elem_in_old_map_is_true() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 3,
        fill: 3,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 3, value: 30 },
        ]),
    };
    hashmap.grow(5); // Trigger a grow
    assert_eq!(hashmap.used, 3); // Expect used count to remain the same after grow
}

#[test]
fn test_grow_with_default_value_elem() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar::default(), // Default value
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(4); // Trigger a grow
    assert_eq!(hashmap.map.as_ref().unwrap()[1].value, 0); // Expect default value still in place
}

#[test]
fn test_grow_when_no_elem_in_old_map() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(1); // Trigger a grow when there are no elements
    assert_eq!(hashmap.map.as_ref().unwrap().iter().all(|elem| elem.value == Default::default()), true); // Verify all are default
}

