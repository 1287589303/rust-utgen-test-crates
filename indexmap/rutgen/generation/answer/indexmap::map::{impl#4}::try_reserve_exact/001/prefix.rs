// Answer 0

#[test]
fn test_try_reserve_exact_zero() {
    let mut index_map = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = index_map.try_reserve_exact(0);
}

#[test]
fn test_try_reserve_exact_small() {
    let mut index_map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    let _ = index_map.try_reserve_exact(1);
}

#[test]
fn test_try_reserve_exact_exact_capacity() {
    let mut index_map = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    let _ = index_map.try_reserve_exact(10);
}

#[test]
fn test_try_reserve_exact_large() {
    let mut index_map = IndexMap::with_capacity_and_hasher(100, RandomState::new());
    let _ = index_map.try_reserve_exact(50);
}

#[test]
#[should_panic]
fn test_try_reserve_exact_over_capacity() {
    let mut index_map = IndexMap::with_capacity_and_hasher(IndexMapCore::MAX_ENTRIES_CAPACITY, RandomState::new());
    let _ = index_map.try_reserve_exact(IndexMapCore::MAX_ENTRIES_CAPACITY + 1);
}

