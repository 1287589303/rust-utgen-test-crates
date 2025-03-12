// Answer 0

#[test]
fn test_truncate_equals_len_with_positive_length() {
    let mut index_map = IndexMapCore::with_capacity(5);
    let hash = HashValue::default();
    index_map.push_entry(hash, 1, "value1");
    index_map.push_entry(hash, 2, "value2");
    
    let len = index_map.len();
    index_map.truncate(len);
}

#[test]
fn test_truncate_equals_len_with_capacity() {
    let mut index_map = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    let hash = HashValue::default();
    
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        index_map.push_entry(hash, i, format!("value{}", i));
    }
    
    let len = index_map.len();
    index_map.truncate(len);
}

