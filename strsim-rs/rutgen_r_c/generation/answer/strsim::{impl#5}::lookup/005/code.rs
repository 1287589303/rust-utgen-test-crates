// Answer 0

#[test]
fn test_lookup_with_default_value_false_first_condition() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 1,
        fill: 4,
        mask: 3, // (4 - 1)
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 2 }, // map[0]
            GrowingHashmapMapElemChar { key: 2, value: 3 }, // map[1]
            GrowingHashmapMapElemChar { key: 3, value: 4 }, // map[2]
            GrowingHashmapMapElemChar { key: 4, value: 5 }, // map[3]
        ]),
    };
    let key = 5; // This key does not exist in the map
    let index = hashmap.lookup(key);
    assert_eq!(index, 1); // Expected index based on the hash calculation
}

#[test]
fn test_lookup_with_default_value_false_second_condition() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 2,
        fill: 4,
        mask: 3, // (4 - 1)
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 2 }, // map[0]
            GrowingHashmapMapElemChar { key: 2, value: 3 }, // map[1]
            GrowingHashmapMapElemChar { key: 3, value: 4 }, // map[2]
            GrowingHashmapMapElemChar { key: 0, value: 0 }, // map[3] 
        ]),
    };
    let key = 4; // This key does not exist in the map
    let index = hashmap.lookup(key);
    assert_eq!(index, 0); // The expected return index
}

#[test]
fn test_lookup_with_default_value_true_after_collision() {
    let mut hashmap = GrowingHashmapChar::<i32> {
        used: 3,
        fill: 8,
        mask: 7, // (8 - 1)
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 2 }, // map[0]
            GrowingHashmapMapElemChar { key: 2, value: 3 }, // map[1]
            GrowingHashmapMapElemChar { key: 3, value: 0 }, // Collision occurs here
            GrowingHashmapMapElemChar { key: 4, value: 5 }, // map[3]
            GrowingHashmapMapElemChar::default(), // map[4]
            GrowingHashmapMapElemChar::default(), // map[5]
            GrowingHashmapMapElemChar::default(), // map[6]
            GrowingHashmapMapElemChar { key: 6, value: 7 }, // map[7]
        ]),
    };
    let key = 3; // This key causes a collision
    let index = hashmap.lookup(key);
    assert_eq!(index, 2); // The expected return index after resolving the collision
}

