// Answer 0

#[test]
fn test_grow_increasing_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 3, // initial mask value
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
            GrowingHashmapMapElemChar::default(), // unused
        ]),
    };
    
    hashmap.grow(5); // min_used > initial mask value
}

#[test]
fn test_grow_double_size() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 2,
        fill: 2,
        mask: 2, // initial mask value
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 4, value: 40 },
            GrowingHashmapMapElemChar { key: 5, value: 50 },
            GrowingHashmapMapElemChar::default(), // unused
        ]),
    };

    hashmap.grow(3); // initial mask value + 1 <= min_used
}

#[test]
fn test_grow_non_default_values() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 5,
        fill: 5,
        mask: 7, // initial mask value
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 6, value: 60 },
            GrowingHashmapMapElemChar { key: 7, value: 70 },
            GrowingHashmapMapElemChar::default(), // unused
            GrowingHashmapMapElemChar::default(), // unused
            GrowingHashmapMapElemChar { key: 8, value: 80 },
            GrowingHashmapMapElemChar::default(), // unused
            GrowingHashmapMapElemChar::default(), // unused
            GrowingHashmapMapElemChar { key: 9, value: 90 },
        ]),
    };
    
    hashmap.grow(8); // min_used > initial mask value
}

#[test]
fn test_grow_existing_elements() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1, // initial mask value
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 10, value: 100 },
            GrowingHashmapMapElemChar::default(), // unused
        ]),
    };

    hashmap.grow(2); // min_used > initial mask value
}

