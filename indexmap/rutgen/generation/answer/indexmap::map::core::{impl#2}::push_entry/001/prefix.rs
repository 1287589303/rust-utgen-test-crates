// Answer 0

#[test]
fn test_push_entry_at_capacity() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    let hash_value = HashValue(1);
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        index_map.push_entry(HashValue(i as usize), i, format!("value{}", i));
    }
    let key = IndexMapCore::MAX_ENTRIES_CAPACITY;
    let value = "new_value".to_string();
    index_map.push_entry(hash_value, key, value);
}

#[test]
fn test_push_entry_exceeding_capacity() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    let hash_value = HashValue(1);
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        index_map.push_entry(HashValue(i as usize), i, format!("value{}", i));
    }
    let key = IndexMapCore::MAX_ENTRIES_CAPACITY + 1;
    let value = "another_value".to_string();
    index_map.push_entry(hash_value, key, value);
}

