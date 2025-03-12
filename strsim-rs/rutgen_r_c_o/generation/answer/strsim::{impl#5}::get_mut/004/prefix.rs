// Answer 0

#[test]
fn test_get_mut_with_initial_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 4,
        fill: 4,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    // Trigger to a default value at a specific key
    let key: u32 = 3;

    let value: &mut i32 = hashmap.get_mut(key);
}

#[test]
fn test_get_mut_with_fill_at_bound() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 4,
        fill: 4,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar::default(),
            GrowingHashmapMapElemChar { key: 1, value: 10 },
            GrowingHashmapMapElemChar { key: 2, value: 20 },
            GrowingHashmapMapElemChar { key: 3, value: 30 },
        ]),
    };

    let key: u32 = 4; // This key should trigger the resize condition

    let value: &mut i32 = hashmap.get_mut(key);
}

