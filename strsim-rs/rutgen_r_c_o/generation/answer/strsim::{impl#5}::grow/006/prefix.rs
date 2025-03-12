// Answer 0

#[test]
fn test_grow_with_min_used_greater_than_mask() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
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
    hashmap.grow(8);
}

#[test]
fn test_grow_with_used_0_after_update() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
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
    hashmap.grow(4);
}

#[test]
fn test_grow_with_elements_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(5);
}

#[test]
fn test_grow_with_default_elements_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar::<i32>::default(),
            GrowingHashmapMapElemChar::<i32>::default(),
        ]),
    };
    hashmap.grow(3);
}

