// Answer 0

#[test]
fn test_keys_empty_map() {
    let map: IndexMap<i32, String, RandomState> = IndexMap::with_hasher(RandomState::new());
    let keys = map.keys();
    let _ = keys.iter; // Testing the function call
}

#[test]
fn test_keys_single_entry() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::with_hasher(RandomState::new());
    map.insert(1, "One".to_string());
    let keys = map.keys();
    let _ = keys.iter; // Testing the function call
}

#[test]
fn test_keys_multiple_entries() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::with_hasher(RandomState::new());
    map.insert(1, "One".to_string());
    map.insert(2, "Two".to_string());
    map.insert(3, "Three".to_string());
    let keys = map.keys();
    let _ = keys.iter; // Testing the function call
}

#[test]
fn test_keys_large_capacity() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::with_capacity_and_hasher(1000, RandomState::new());
    for i in 0..1000 {
        map.insert(i, format!("Value {}", i));
    }
    let keys = map.keys();
    let _ = keys.iter; // Testing the function call
}

#[test]
fn test_keys_boundary_conditions() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    assert!(map.is_empty()); // Just to verify before calling keys
    let keys = map.keys();
    let _ = keys.iter; // Testing the function call

    map.insert(0, "Zero".to_string());
    let keys = map.keys();
    let _ = keys.iter; // Testing the function call after adding an entry
}

