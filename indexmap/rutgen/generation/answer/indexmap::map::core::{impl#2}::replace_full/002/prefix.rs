// Answer 0

#[test]
fn test_replace_full_with_empty_entries() {
    let mut index_map: IndexMapCore<i32, String> = IndexMapCore::new();
    let hash = HashValue(123);
    let key = 1;
    let value = "value_1".to_string();
    index_map.replace_full(hash, key, value);
}

#[test]
fn test_replace_full_with_single_existing_entry() {
    let mut index_map: IndexMapCore<i32, String> = IndexMapCore::new();
    let hash_existing = HashValue(456);
    let key_existing = 2;
    let value_existing = "value_2".to_string();
    index_map.replace_full(hash_existing, key_existing, value_existing);
    
    let hash_new = HashValue(123);
    let key_new = 3; // distinct from key_existing
    let value_new = "value_3".to_string();
    index_map.replace_full(hash_new, key_new, value_new);
}

#[test]
fn test_replace_full_with_max_capacity() {
    let mut index_map: IndexMapCore<i32, String> = IndexMapCore::with_capacity(IndexMapCore::<i32, String>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<i32, String>::MAX_ENTRIES_CAPACITY {
        let hash = HashValue(i as usize);
        let key = i as i32;
        let value = format!("value_{}", i);
        index_map.replace_full(hash, key, value);
    }
    
    let hash_overflow = HashValue(IndexMapCore::<i32, String>::MAX_ENTRIES_CAPACITY);
    let key_overflow = IndexMapCore::<i32, String>::MAX_ENTRIES_CAPACITY as i32; // distinct
    let value_overflow = "overflow_value".to_string();
    index_map.replace_full(hash_overflow, key_overflow, value_overflow);
}

