// Answer 0

#[test]
fn test_lookup_key_found() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 }, // Default
            GrowingHashmapMapElemChar { key: 1, value: 42 }, // Existing key
        ]),
    };

    let key: u32 = 1;
    let index = hashmap.lookup(key);
    assert_eq!(index, 1);
}

#[test]
fn test_lookup_key_collides() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 }, // Default
            GrowingHashmapMapElemChar { key: 2, value: 100 }, // Existing key causing collision
        ]),
    };

    let key: u32 = 2;
    let index = hashmap.lookup(key);
    assert_eq!(index, 1); // Should resolve to the same index due to collision
}

#[test]
fn test_lookup_non_default_value() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 1,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 }, // Default
            GrowingHashmapMapElemChar { key: 3, value: 50 }, // Non-default value
        ]),
    };

    let key: u32 = 3;
    let index = hashmap.lookup(key);
    assert_eq!(index, 1);
}

