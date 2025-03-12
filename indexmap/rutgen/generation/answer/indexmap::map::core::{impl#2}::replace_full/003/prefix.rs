// Answer 0

#[test]
fn test_replace_full_occupied_entry() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash = HashValue(123);
    let key = 1;
    let value = "existing_value".to_string();

    // Insert an initial entry to ensure it can be replaced
    map.replace_full(hash, key, value.clone());

    // Prepare new key and value for replacement
    let new_key = 1;
    let new_value = "new_value".to_string();

    // Call the function under test
    map.replace_full(hash, new_key, new_value);
}

#[test]
fn test_replace_full_entry_with_different_value() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash = HashValue(456);
    let key = 2;
    let value = "old_value".to_string();

    // Insert the entry to ensure it exists
    map.replace_full(hash, key, value.clone());

    // Prepare new key and value for replacement
    let new_value = "updated_value".to_string();

    // Call the function under test
    map.replace_full(hash, key, new_value);
}

#[test]
fn test_replace_full_with_same_key() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash = HashValue(789);
    let key = 3;
    let value = "value1".to_string();

    // Insert the initial entry
    map.replace_full(hash, key, value.clone());

    // Call replace_full with the same key but a different value
    let new_value = "value2".to_string();
    map.replace_full(hash, key, new_value);
}

