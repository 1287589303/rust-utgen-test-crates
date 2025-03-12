// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero() {
    let hash_builder = RandomState::new();
    let map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(0, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_one() {
    let hash_builder = RandomState::new();
    let map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(1, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_maximum() {
    let hash_builder = RandomState::new();
    let map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(1000, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_large_value() {
    let hash_builder = RandomState::new();
    let map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity_and_hasher(usize::MAX, hash_builder);
}

