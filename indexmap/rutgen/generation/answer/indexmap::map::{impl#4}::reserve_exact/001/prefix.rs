// Answer 0

#[test]
fn test_reserve_exact_zero_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.reserve_exact(0);
}

#[test]
fn test_reserve_exact_minimum_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.reserve_exact(1);
}

#[test]
fn test_reserve_exact_boundary_capacity() {
    let max_capacity = IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY;
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(max_capacity, RandomState::new());
    map.reserve_exact(max_capacity);
}

#[test]
fn test_reserve_exact_mid_capacity() {
    let mid_capacity = IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY / 2;
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(mid_capacity, RandomState::new());
    map.reserve_exact(mid_capacity);
}

