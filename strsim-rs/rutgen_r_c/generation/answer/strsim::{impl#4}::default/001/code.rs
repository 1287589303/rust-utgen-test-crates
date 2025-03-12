// Answer 0

#[test]
fn test_default_growing_hashmap_char() {
    let result: GrowingHashmapChar<i32> = GrowingHashmapChar::default();
    assert_eq!(result.used, 0);
    assert_eq!(result.fill, 0);
    assert_eq!(result.mask, -1);
    assert_eq!(result.map, None);
}

#[test]
fn test_default_growing_hashmap_char_with_string() {
    let result: GrowingHashmapChar<String> = GrowingHashmapChar::default();
    assert_eq!(result.used, 0);
    assert_eq!(result.fill, 0);
    assert_eq!(result.mask, -1);
    assert_eq!(result.map, None);
}

#[test]
fn test_default_growing_hashmap_char_with_vec() {
    let result: GrowingHashmapChar<Vec<i32>> = GrowingHashmapChar::default();
    assert_eq!(result.used, 0);
    assert_eq!(result.fill, 0);
    assert_eq!(result.mask, -1);
    assert_eq!(result.map, None);
}

