// Answer 0

#[test]
fn test_with_hasher_random_state() {
    let hash_builder = std::collections::hash_map::RandomState::new();
    let map: IndexMap<usize, String, _> = IndexMap::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_zero_capacity() {
    let hash_builder = std::collections::hash_map::RandomState::new();
    let map: IndexMap<usize, String, _> = IndexMap::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_max_capacity() {
    let hash_builder = std::collections::hash_map::RandomState::new();
    let map: IndexMap<usize, String, _> = IndexMap::with_capacity_and_hasher(usize::MAX, hash_builder);
}

#[test]
fn test_with_hasher_empty_random_state() {
    let hash_builder = std::collections::hash_map::RandomState::new();
    let map: IndexMap<usize, String, _> = IndexMap::with_capacity_and_hasher(0, hash_builder);
}

