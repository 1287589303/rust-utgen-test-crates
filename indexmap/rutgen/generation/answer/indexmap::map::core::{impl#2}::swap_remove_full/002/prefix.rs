// Answer 0

#[test]
fn test_swap_remove_full_valid_entry() {
    let mut map: IndexMapCore<u32, String> = IndexMapCore::new();
    let hash_val = HashValue(42);
    let key = 1;
    let value = "value".to_string();

    map.insert_full(hash_val, key, value.clone());

    let result = map.swap_remove_full(hash_val, &key);
    // The result is expected to be Some((0, key, value)) since it's the only entry.
}

#[test]
fn test_swap_remove_full_with_multiple_entries() {
    let mut map: IndexMapCore<u32, String> = IndexMapCore::new();
    let hash_val1 = HashValue(1);
    let hash_val2 = HashValue(2);
    let key1 = 1;
    let key2 = 2;
    let value1 = "value1".to_string();
    let value2 = "value2".to_string();

    map.insert_full(hash_val1, key1, value1.clone());
    map.insert_full(hash_val2, key2, value2.clone());

    let result = map.swap_remove_full(hash_val1, &key1);
    // The result is expected to be Some((0, key1, value1)), removing the first entry.
}

#[test]
fn test_swap_remove_full_boundary_case() {
    let mut map: IndexMapCore<u32, String> = IndexMapCore::with_capacity(1);
    let hash_val = HashValue(10);
    let key = 10;
    let value = "boundary_value".to_string();

    map.insert_full(hash_val, key, value.clone());

    let result = map.swap_remove_full(hash_val, &key);
    // The result is expected to be Some((0, key, value)), as it's the only entry present.
}

