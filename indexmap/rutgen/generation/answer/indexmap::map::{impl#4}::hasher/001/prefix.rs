// Answer 0

#[test]
fn test_hasher_with_zero_capacity() {
    use std::collections::hash_map::RandomState;
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let hasher = map.hasher();
}

#[test]
fn test_hasher_with_one_capacity() {
    use std::collections::hash_map::RandomState;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.core.entries.push((1, 10)); // Simulating adding an entry
    let hasher = map.hasher();
}

#[test]
fn test_hasher_with_ten_capacity() {
    use std::collections::hash_map::RandomState;
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    for i in 0..10 {
        map.core.entries.push((i, i * 10)); // Simulating adding entries
    }
    let hasher = map.hasher();
}

#[test]
fn test_hasher_with_custom_hasher() {
    use std::collections::hash_map::RandomState;
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }
    let mut map: IndexMap<i32, i32, CustomHasher> = IndexMap::with_capacity_and_hasher(0, CustomHasher);
    let hasher = map.hasher();
}

#[test]
fn test_hasher_with_empty_map() {
    use std::collections::hash_map::RandomState;
    let map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    assert!(map.is_empty());
    let hasher = map.hasher();
}

