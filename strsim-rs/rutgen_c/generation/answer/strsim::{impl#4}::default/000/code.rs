// Answer 0

#[test]
fn test_default_growth_hashmap_char() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar::default();
    assert_eq!(hashmap.used, 0);
    assert_eq!(hashmap.fill, 0);
    assert_eq!(hashmap.mask, -1);
    assert!(hashmap.map.is_none());
}

#[test]
fn test_default_growth_hashmap_char_with_different_value_type() {
    let hashmap: GrowingHashmapChar<String> = GrowingHashmapChar::default();
    assert_eq!(hashmap.used, 0);
    assert_eq!(hashmap.fill, 0);
    assert_eq!(hashmap.mask, -1);
    assert!(hashmap.map.is_none());
}

