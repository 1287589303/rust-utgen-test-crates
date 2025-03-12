// Answer 0

#[test]
fn test_lookup_with_non_default_values_and_non_matching_key_just_below_mask() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 16,
        mask: 15,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 2 },
            GrowingHashmapMapElemChar { key: 2, value: 3 },
            GrowingHashmapMapElemChar { key: 3, value: 4 },
            GrowingHashmapMapElemChar { key: 4, value: 5 },
            GrowingHashmapMapElemChar { key: 5, value: 6 },
            GrowingHashmapMapElemChar { key: 6, value: 7 },
            GrowingHashmapMapElemChar { key: 7, value: 8 },
            GrowingHashmapMapElemChar { key: 8, value: 9 },
            GrowingHashmapMapElemChar { key: 9, value: 10 },
            GrowingHashmapMapElemChar { key: 10, value: 11 },
            GrowingHashmapMapElemChar { key: 11, value: 12 },
            GrowingHashmapMapElemChar { key: 12, value: 13 },
            GrowingHashmapMapElemChar { key: 13, value: 14 },
            GrowingHashmapMapElemChar { key: 14, value: 15 },
            GrowingHashmapMapElemChar { key: 15, value: 16 },
            GrowingHashmapMapElemChar { key: 16, value: 17 },
        ]),
    };
    let result = hashmap.lookup(1); // Non-matching key, traverses
}

#[test]
fn test_lookup_with_non_default_values_and_non_matching_key_at_mask() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 16,
        mask: 15,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 1 },
            GrowingHashmapMapElemChar { key: 1, value: 2 },
            GrowingHashmapMapElemChar { key: 2, value: 3 },
            GrowingHashmapMapElemChar { key: 3, value: 4 },
            GrowingHashmapMapElemChar { key: 4, value: 5 },
            GrowingHashmapMapElemChar { key: 5, value: 6 },
            GrowingHashmapMapElemChar { key: 6, value: 7 },
            GrowingHashmapMapElemChar { key: 7, value: 8 },
            GrowingHashmapMapElemChar { key: 8, value: 9 },
            GrowingHashmapMapElemChar { key: 9, value: 10 },
            GrowingHashmapMapElemChar { key: 10, value: 11 },
            GrowingHashmapMapElemChar { key: 11, value: 12 },
            GrowingHashmapMapElemChar { key: 12, value: 13 },
            GrowingHashmapMapElemChar { key: 13, value: 14 },
            GrowingHashmapMapElemChar { key: 14, value: 15 },
            GrowingHashmapMapElemChar { key: 15, value: 16 },
        ]),
    };
    let result = hashmap.lookup(2); // Non-matching key, traverses
}

#[test]
fn test_lookup_with_high_collision_key() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 16,
        mask: 15,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 1 },
            GrowingHashmapMapElemChar { key: 1, value: 2 },
            GrowingHashmapMapElemChar { key: 1, value: 0 }, // Collision
            GrowingHashmapMapElemChar { key: 3, value: 4 },
            GrowingHashmapMapElemChar { key: 4, value: 5 },
            GrowingHashmapMapElemChar { key: 5, value: 6 },
            GrowingHashmapMapElemChar { key: 6, value: 7 },
            GrowingHashmapMapElemChar { key: 7, value: 8 },
            GrowingHashmapMapElemChar { key: 8, value: 9 },
            GrowingHashmapMapElemChar { key: 9, value: 10 },
            GrowingHashmapMapElemChar { key: 10, value: 11 },
            GrowingHashmapMapElemChar { key: 11, value: 12 },
            GrowingHashmapMapElemChar { key: 12, value: 13 },
            GrowingHashmapMapElemChar { key: 13, value: 14 },
            GrowingHashmapMapElemChar { key: 14, value: 15 },
            GrowingHashmapMapElemChar { key: 15, value: 16 },
        ]),
    };
    let result = hashmap.lookup(1); // Non-matching key causing collision
} 

#[test]
fn test_lookup_with_max_u32_key() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 16,
        mask: 15,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 1 },
            GrowingHashmapMapElemChar { key: 1, value: 2 },
            GrowingHashmapMapElemChar { key: 2, value: 3 },
            GrowingHashmapMapElemChar { key: 3, value: 4 },
            GrowingHashmapMapElemChar { key: 4, value: 5 },
            GrowingHashmapMapElemChar { key: 5, value: 6 },
            GrowingHashmapMapElemChar { key: 6, value: 7 },
            GrowingHashmapMapElemChar { key: 7, value: 8 },
            GrowingHashmapMapElemChar { key: 8, value: 9 },
            GrowingHashmapMapElemChar { key: 9, value: 10 },
            GrowingHashmapMapElemChar { key: 10, value: 11 },
            GrowingHashmapMapElemChar { key: 11, value: 12 },
            GrowingHashmapMapElemChar { key: 12, value: 13 },
            GrowingHashmapMapElemChar { key: 13, value: 14 },
            GrowingHashmapMapElemChar { key: 14, value: 15 },
            GrowingHashmapMapElemChar { key: 15, value: 16 },
        ]),
    };
    let result = hashmap.lookup(u32::MAX); // Testing with a maximum key
}

#[test]
fn test_lookup_with_key_just_above_mask() {
    let mut hashmap = GrowingHashmapChar::<usize> {
        used: 0,
        fill: 16,
        mask: 15,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 1 },
            GrowingHashmapMapElemChar { key: 1, value: 2 },
            GrowingHashmapMapElemChar { key: 2, value: 3 },
            GrowingHashmapMapElemChar { key: 3, value: 4 },
            GrowingHashmapMapElemChar { key: 4, value: 5 },
            GrowingHashmapMapElemChar { key: 5, value: 6 },
            GrowingHashmapMapElemChar { key: 6, value: 7 },
            GrowingHashmapMapElemChar { key: 7, value: 8 },
            GrowingHashmapMapElemChar { key: 8, value: 9 },
            GrowingHashmapMapElemChar { key: 9, value: 10 },
            GrowingHashmapMapElemChar { key: 10, value: 11 },
            GrowingHashmapMapElemChar { key: 11, value: 12 },
            GrowingHashmapMapElemChar { key: 12, value: 13 },
            GrowingHashmapMapElemChar { key: 13, value: 14 },
            GrowingHashmapMapElemChar { key: 14, value: 15 },
            GrowingHashmapMapElemChar { key: 15, value: 16 },
        ]),
    };
    let result = hashmap.lookup(16); // Non-matching key just above the mask
}

