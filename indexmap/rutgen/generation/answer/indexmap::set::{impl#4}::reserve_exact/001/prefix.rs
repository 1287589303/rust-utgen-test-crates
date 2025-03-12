// Answer 0

#[test]
fn test_reserve_exact_with_zero_additional() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    index_set.reserve_exact(0);
}

#[test]
fn test_reserve_exact_with_small_additional() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    index_set.reserve_exact(5);
}

#[test]
fn test_reserve_exact_with_large_additional() {
    let mut index_set: super::IndexSet<i32, std::collections::hash_map::RandomState> = 
        super::IndexSet::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    index_set.reserve_exact(usize::MAX);
}

