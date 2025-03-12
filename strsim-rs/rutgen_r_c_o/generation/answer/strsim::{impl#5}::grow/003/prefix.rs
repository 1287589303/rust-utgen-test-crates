// Answer 0

#[test]
fn test_grow_min_used_zero() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 1]),
    };
    hashmap.grow(0);
}

#[test]
fn test_grow_min_used_equals_new_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(2);
}

#[test]
fn test_grow_min_used_greater_than_new_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar { key: 2, value: 84 },
        ]),
    };
    hashmap.grow(3);
}

#[test]
fn test_grow_old_map_contains_non_default_values() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 42 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    hashmap.grow(2);
}

#[test]
fn test_grow_old_map_all_default_values() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: Some(vec![GrowingHashmapMapElemChar::default(); 2]),
    };
    hashmap.grow(1);
}

