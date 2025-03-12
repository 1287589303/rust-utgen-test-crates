// Answer 0

#[test]
fn test_with_hasher_default() {
    use std::collections::hash_map::RandomState;
    let hash_builder = RandomState::new();
    let index_set: IndexSet<(), RandomState> = IndexSet::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_non_default() {
    use std::collections::hash_map::RandomState;
    let hash_builder = RandomState::new();
    let index_set: IndexSet<(), RandomState> = IndexSet::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_min_capacity() {
    use std::collections::hash_map::RandomState;
    let hash_builder = RandomState::new();
    let index_set: IndexSet<(), RandomState> = IndexSet::with_hasher(hash_builder);
}

#[test]
fn test_with_hasher_large_capacity() {
    use std::collections::hash_map::RandomState;
    let hash_builder = RandomState::new();
    let index_set: IndexSet<(), RandomState> = IndexSet::with_hasher(hash_builder);
}

