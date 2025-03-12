// Answer 0

#[test]
fn test_capacity_zero() {
    let index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> =
        super::IndexSet::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    let _cap = index_set.capacity();
}

#[test]
fn test_capacity_one() {
    let index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> =
        super::IndexSet::with_capacity_and_hasher(1, std::collections::hash_map::RandomState::new());
    let _cap = index_set.capacity();
}

#[test]
fn test_capacity_large() {
    let index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> =
        super::IndexSet::with_capacity_and_hasher(1000, std::collections::hash_map::RandomState::new());
    let _cap = index_set.capacity();
}

#[test]
fn test_capacity_after_addition() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> =
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    index_set.reserve(5);
    let _cap = index_set.capacity();
}

