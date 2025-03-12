// Answer 0

#[test]
fn test_with_capacity_and_hasher_positive() {
    let n = 1;
    let hash_builder = RandomState::new(); // Assuming RandomState is a valid hasher
    let _map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

#[test]
fn test_with_capacity_and_hasher_boundary() {
    let n = IndexMapCore::MAX_ENTRIES_CAPACITY;
    let hash_builder = RandomState::new(); // Assuming RandomState is a valid hasher
    let _map = IndexMap::with_capacity_and_hasher(n, hash_builder);
}

