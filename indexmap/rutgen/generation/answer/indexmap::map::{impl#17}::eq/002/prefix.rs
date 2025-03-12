// Answer 0

#[test]
fn test_eq_same_length_empty_map() {
    let map1: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let map2: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    assert!(map1.eq(&map2));
}

#[test]
fn test_eq_same_length_single_element() {
    let mut map1: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map1.insert(1, "a");
    let mut map2: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map2.insert(1, "a");
    assert!(map1.eq(&map2));
}

#[test]
fn test_eq_same_length_multiple_elements() {
    let mut map1: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map1.insert(1, "a");
    map1.insert(2, "b");
    let mut map2: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(3, RandomState::new());
    map2.insert(1, "a");
    map2.insert(2, "b");
    assert!(map1.eq(&map2));
}

#[test]
fn test_eq_same_length_different_values() {
    let mut map1: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map1.insert(1, "a");
    map1.insert(2, "b");
    let mut map2: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map2.insert(1, "a");
    map2.insert(2, "c");
    assert!(!map1.eq(&map2));
}

#[test]
fn test_eq_same_length_keys_different_order() {
    let mut map1: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map1.insert(1, "a");
    map1.insert(2, "b");
    let mut map2: IndexMap<u32, &str, RandomState> = IndexMap::with_capacity_and_hasher(2, RandomState::new());
    map2.insert(2, "b");
    map2.insert(1, "a");
    assert!(map1.eq(&map2));
}

