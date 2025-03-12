// Answer 0

#[test]
fn test_grow_new_size_greater_than_min_used() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3, // Example mask
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 1 },
            GrowingHashmapMapElemChar { key: 2, value: 2 },
            GrowingHashmapMapElemChar { key: 3, value: 3 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    hashmap.grow(2); // min_used < new_size (expected new_size: 4)
    
    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.used, 0);
}

#[test]
fn test_grow_elem_in_old_map_true_and_has_non_default_value() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 1 },
            GrowingHashmapMapElemChar { key: 2, value: 0 }, // Default value
            GrowingHashmapMapElemChar { key: 3, value: 3 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    hashmap.grow(2);
    
    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.map.as_ref().expect("map created above").len(), 8); // New size should be 8
    assert_eq!(hashmap.map.as_ref().expect("map created above")[1].value, 1); // Elem has non-default value
}

#[test]
fn test_grow_no_elements_in_old_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: 0 }, // Default
            GrowingHashmapMapElemChar { key: 2, value: 0 }, // Default
            GrowingHashmapMapElemChar { key: 3, value: 0 }, // Default
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    hashmap.grow(2);
    
    assert_eq!(hashmap.used, 0);
}

#[test]
fn test_grow_elem_in_old_map_false() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 3,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar { key: 0, value: 0 },
            GrowingHashmapMapElemChar::default(),
        ]),
    };
    
    hashmap.grow(2); // min_used < new_size (expected new_size: 4)
    
    assert_eq!(hashmap.mask, 3);
    assert_eq!(hashmap.map.as_ref().unwrap().iter().filter(|x| x.value != Default::default()).count(), 0);
}

