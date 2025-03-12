// Answer 0

#[test]
fn test_lookup_non_default_value_first_check_fail_on_key() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 100 },
            GrowingHashmapMapElemChar { key: 2, value: 200 },
            GrowingHashmapMapElemChar { key: 3, value: 300 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let key = 4; // A key that does not exist in the hashmap
    let index = hashmap.lookup(key);
}

#[test]
fn test_lookup_non_default_value_with_collision() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 100 },
            GrowingHashmapMapElemChar { key: 2, value: 200 },
            GrowingHashmapMapElemChar { key: 5, value: 300 }, // causes a collision with mask
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    let key = 6; // A key that does not exist in the hashmap
    let index = hashmap.lookup(key);
}

#[test]
fn test_lookup_non_default_value_and_empty_space() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 100 },
            GrowingHashmapMapElemChar { key: 2, value: 200 },
            GrowingHashmapMapElemChar { key: 3, value: 300 },
            GrowingHashmapMapElemChar::default(), // Empty space
        ]),
    };
    let key = 4; // A key that does not exist in the hashmap
    let index = hashmap.lookup(key);
}

