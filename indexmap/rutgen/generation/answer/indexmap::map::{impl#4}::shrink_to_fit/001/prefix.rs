// Answer 0

#[test]
fn test_shrink_to_fit_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_non_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_full_capacity() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::MAX_ENTRIES_CAPACITY, RandomState::new());
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        map.insert(i, i);
    }
    map.shrink_to_fit();
}

#[test]
fn test_shrink_to_fit_after_removal() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 1);
    map.insert(2, 2);
    map.remove(&1);
    map.shrink_to_fit();
}

