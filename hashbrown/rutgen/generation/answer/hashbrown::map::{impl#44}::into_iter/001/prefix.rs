// Answer 0

#[test]
fn test_empty_hash_map_iter_mut() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let mut iter = map.iter_mut();
    drop(iter); // Ensure it compiles with no entries.
}

#[test]
fn test_single_entry_hash_map_iter_mut() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("key1", 10);
    let mut iter = map.iter_mut();
    if let Some((key, value)) = iter.next() {
        *value *= 2; // Mutate value.
        assert_eq!(*value, 20);
    }
}

#[test]
fn test_multiple_entries_hash_map_iter_mut() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("key1", 5);
    map.insert("key2", 10);
    map.insert("key3", 15);
    
    let mut iter = map.iter_mut();
    while let Some((_, value)) = iter.next() {
        *value *= 2; // Mutate values
    }
}

#[test]
fn test_full_capacity_hash_map_iter_mut() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, 1);
    map.insert(2, 2);
    
    let mut iter = map.iter_mut();
    while let Some((_, value)) = iter.next() {
        *value += 1; // Increment value by 1.
    }
}

