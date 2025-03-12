// Answer 0

#[test]
fn test_capacity_empty_map() {
    let map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let _ = map.capacity();
}

#[test]
fn test_capacity_single_entry() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    let _ = map.capacity();
}

#[test]
fn test_capacity_multiple_entries() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let _ = map.capacity();
}

#[test]
fn test_capacity_at_max_capacity() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity_and_hasher(IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY, RandomState::new());
    for i in 0..IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY {
        map.insert(i, i);
    }
    let _ = map.capacity();
}

