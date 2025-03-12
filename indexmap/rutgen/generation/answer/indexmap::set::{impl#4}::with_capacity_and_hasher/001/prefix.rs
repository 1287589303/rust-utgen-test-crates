// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    use std::collections::hash_map::RandomState;
    let n = 0;
    let hasher = RandomState::new();
    let set: super::IndexSet<(), RandomState> = super::IndexSet::with_capacity_and_hasher(n, hasher);
}

#[test]
fn test_with_capacity_and_hasher_positive_capacity() {
    use std::collections::hash_map::RandomState;
    let n = 10;
    let hasher = RandomState::new();
    let set: super::IndexSet<(), RandomState> = super::IndexSet::with_capacity_and_hasher(n, hasher);
}

#[test]
fn test_with_capacity_and_hasher_custom_hasher() {
    use std::collections::hash_map::{RandomState, DefaultHasher};
    let n = 5;
    let hasher = DefaultHasher::new();
    let set: super::IndexSet<(), DefaultHasher> = super::IndexSet::with_capacity_and_hasher(n, hasher);
}

