// Answer 0

#[test]
fn test_lookup_with_collision_resolution() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 }, // Example values
            GrowingHashmapMapElemChar { key: 2, value: 10 }, // map[1] with key not equal to lookup key
            GrowingHashmapMapElemChar { key: 3, value: 15 }, // map[2] with key not equal to lookup key
            GrowingHashmapMapElemChar { key: 4, value: 20 }, // map[3] for initial probing
        ]),
    };

    let key: u32 = 2; // key to lookup
    hashmap.lookup(key);
}

#[test]
fn test_lookup_with_greater_key() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 5 }, // map[0]
            GrowingHashmapMapElemChar { key: 3, value: 10 }, // key 3 not equal to lookup key
            GrowingHashmapMapElemChar { key: 4, value: 15 }, // key 4 not equal to lookup key
            GrowingHashmapMapElemChar { key: 5, value: 20 }, // key to conflict on
        ]),
    };

    let key: u32 = 3; // key to lookup
    hashmap.lookup(key);
}

#[test]
fn test_lookup_with_edge_key() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 1 }, // First element with different key
            GrowingHashmapMapElemChar { key: 2, value: 2 }, // map[1] with key not equal to lookup key
            GrowingHashmapMapElemChar { key: 4, value: 4 }, // map[2]
            GrowingHashmapMapElemChar { key: 8, value: 8 }, // Probing slot
            GrowingHashmapMapElemChar { key: 16, value: 16 }, // Probing slot
            GrowingHashmapMapElemChar { key: 3, value: 15 }, // map[5] with key equal to lookup key
            GrowingHashmapMapElemChar { key: 6, value: 6 }, // Further slots
            GrowingHashmapMapElemChar { key: 7, value: 7 }, // Further slots
        ]),
    };

    let key: u32 = 3; // key to lookup
    hashmap.lookup(key);
}

