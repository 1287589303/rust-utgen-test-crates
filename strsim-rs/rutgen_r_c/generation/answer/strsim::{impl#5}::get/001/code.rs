// Answer 0

#[test]
fn test_get_when_map_is_none_returns_default() {
    let hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    let result = hashmap.get(42);
    assert_eq!(result, 0);
}

#[test]
fn test_get_when_map_is_some_returns_correct_value() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 42, value: 10 }]),
    };
    let result = hashmap.get(42);
    assert_eq!(result, 10);
}

#[test]
fn test_get_when_key_not_found_returns_default() {
    let mut hashmap: GrowingHashmapChar<i32> = GrowingHashmapChar {
        used: 1,
        fill: 1,
        mask: 1,
        map: Some(vec![GrowingHashmapMapElemChar { key: 42, value: 10 }]),
    };
    let result = hashmap.get(99);
    assert_eq!(result, 0);
}

