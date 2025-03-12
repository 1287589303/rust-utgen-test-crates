// Answer 0

#[test]
fn test_try_reserve_zero() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.try_reserve(0).unwrap();
}

#[test]
fn test_try_reserve_one() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.try_reserve(1).unwrap();
}

#[test]
fn test_try_reserve_max_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY, RandomState::new());
    map.try_reserve(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY).unwrap();
}

#[should_panic]
#[test]
fn test_try_reserve_exceed_max_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY, RandomState::new());
    map.try_reserve(IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY + 1).unwrap();
}

