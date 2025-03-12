// Answer 0

#[test]
fn test_shrink_to_zero_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.shrink_to(0);
}

#[test]
fn test_shrink_to_mid_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.shrink_to(5);
}

#[test]
fn test_shrink_to_max_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMap::MAX_ENTRIES_CAPACITY, RandomState::new());
    map.shrink_to(IndexMap::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_shrink_to_exact_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(8, RandomState::new());
    map.shrink_to(8);
}

#[test]
#[should_panic]
fn test_shrink_to_exceeding_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(10, RandomState::new());
    map.shrink_to(11); // This should panic as it exceeds MAX_ENTRIES_CAPACITY
}

