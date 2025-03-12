// Answer 0

#[test]
fn test_clear_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.clear();
}

#[test]
fn test_clear_single_entry() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.insert_full(1, 1, 10);
    index_map.clear();
}

#[test]
fn test_clear_max_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        index_map.insert_full(1, i, i as usize);
    }
    index_map.clear();
}

