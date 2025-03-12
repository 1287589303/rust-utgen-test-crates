// Answer 0

#[test]
fn test_insert_full_occupied_case() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value = HashValue(5);
    let key = 1;
    let initial_value = "Initial".to_string();

    // Insert an initial value that will be replaced later
    map.insert_full(hash_value, key, initial_value.clone());

    let new_value = "Updated".to_string();
    let result = map.insert_full(hash_value, key, new_value);

    // Calling the function to ensure it operates as intended without assertions
    let _ = result;
}

#[test]
fn test_insert_full_occupied_case_different_hash() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value = HashValue(10);
    let key = 2;
    let initial_value = "Existing".to_string();

    map.insert_full(hash_value, key, initial_value.clone());

    let new_value = "New Value".to_string();
    let result = map.insert_full(hash_value, key, new_value);

    let _ = result;
}

#[test]
fn test_insert_full_multiple_entries() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash_value1 = HashValue(3);
    let key1 = 1;
    let initial_value1 = "First".to_string();
    let initial_value2 = "Second".to_string();

    map.insert_full(hash_value1, key1, initial_value1.clone());

    let hash_value2 = HashValue(5);
    let key2 = 2;
    map.insert_full(hash_value2, key2, initial_value2.clone());

    let new_value = "First Updated".to_string();
    let result = map.insert_full(hash_value1, key1, new_value);

    let _ = result;
}

