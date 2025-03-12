// Answer 0

#[test]
fn test_clear_empty_index_map() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    index_map.clear();
}

#[test]
fn test_clear_single_entry_index_map() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    index_map.insert(1, 10);
    index_map.clear();
}

#[test]
fn test_clear_multiple_entries_index_map() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    index_map.clear();
}

#[test]
fn test_clear_max_capacity_index_map() {
    let max_capacity = IndexMapCore::<i32, i32>::MAX_ENTRIES_CAPACITY;
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::with_capacity_and_hasher(max_capacity, RandomState::new());
    for i in 0..max_capacity {
        index_map.insert(i, i * 10);
    }
    index_map.clear();
}

