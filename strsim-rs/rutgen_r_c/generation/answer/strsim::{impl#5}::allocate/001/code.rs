// Answer 0

#[test]
fn test_allocate_initializes_mask_and_map() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    hashmap.allocate();
    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    let map = hashmap.map.as_ref().unwrap();
    assert_eq!(map.len(), 8);
}

#[test]
fn test_allocate_resets_on_multiple_calls() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    hashmap.allocate();
    hashmap.allocate();
    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    let map = hashmap.map.as_ref().unwrap();
    assert_eq!(map.len(), 8);
}

#[test]
fn test_allocate_with_different_value_types() {
    let mut hashmap: GrowingHashmapChar<String> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    hashmap.allocate();
    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    let map = hashmap.map.as_ref().unwrap();
    assert_eq!(map.len(), 8);
}

