// Answer 0

#[test]
fn test_lookup_with_default_value_at_index() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 0, // map length of 1, mask is 0
        map: Some(vec![GrowingHashmapMapElemChar { key: 0, value: Default::default() }]), // only index 0 has default value
    };
    let key: u32 = 0;
    let index = hashmap.lookup(key);
}

#[test]
fn test_lookup_with_default_value_at_first_index() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1, // map length of 2, mask is 1
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: Default::default() }, // index 0 has default value
            GrowingHashmapMapElemChar { key: 1, value: 42 }, // index 1 has non-default value
        ]),
    };
    let key: u32 = 0;
    let index = hashmap.lookup(key);
}

#[test]
fn test_lookup_with_default_value_at_last_index() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1, // map length of 2, mask is 1
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 42 }, // index 0 has non-default value
            GrowingHashmapMapElemChar { key: 1, value: Default::default() }, // index 1 has default value
        ]),
    };
    let key: u32 = 1;
    let index = hashmap.lookup(key);
}

#[test]
fn test_lookup_with_random_key_and_default_values() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3, // map length of 4, mask is 3
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: Default::default() }, // index 0 has default value
            GrowingHashmapMapElemChar { key: 1, value: Default::default() }, // index 1 has default value
            GrowingHashmapMapElemChar { key: 2, value: 42 }, // index 2 has non-default value
            GrowingHashmapMapElemChar { key: 3, value: 99 }, // index 3 has non-default value
        ]),
    };
    let key: u32 = 0;
    let index = hashmap.lookup(key);
}

#[test]
fn test_lookup_with_large_key() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 7, // map length of 8, mask is 7
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 42 }, // index 0 has non-default value
            GrowingHashmapMapElemChar { key: 1, value: 99 }, // index 1 has non-default value
            GrowingHashmapMapElemChar { key: 2, value: Default::default() }, // index 2 has default value
            GrowingHashmapMapElemChar { key: 3, value: 85 }, // index 3 has non-default value
            GrowingHashmapMapElemChar { key: 4, value: 100 }, // index 4 has non-default value
            GrowingHashmapMapElemChar { key: 5, value: Default::default() }, // index 5 has default value
            GrowingHashmapMapElemChar { key: 6, value: 57 }, // index 6 has non-default value
            GrowingHashmapMapElemChar { key: 7, value: Default::default() }, // index 7 has default value
        ]),
    };
    let key: u32 = 2;
    let index = hashmap.lookup(key);
}

