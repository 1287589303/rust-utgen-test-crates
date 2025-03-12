// Answer 0

#[test]
fn test_reserve_zero_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    index_map.reserve(0);
}

#[test]
fn test_reserve_small_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    index_map.reserve(1);
}

#[test]
fn test_reserve_boundary_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    index_map.reserve(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_reserve_large_capacity() {
    let max_capacity = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY;
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(max_capacity, RandomState::new());
    index_map.reserve(max_capacity);
}

#[test]
fn test_reserve_exceed_capacity() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let additional = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY;
    index_map.reserve(additional);
}

