// Answer 0

#[test]
fn test_and_modify_on_occupied_entry() {
    use hashbrown::HashMap;

    // Initialize a hash map and insert a value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key1", 10);

    // Access the Entry and apply a modification function
    let entry = map.entry("key1");
    let modified_entry = entry.and_modify(|v| *v += 5);

    // The modified_entry should still be occupied
    let _occupied_entry = match modified_entry {
        Entry::Occupied(entry) => entry,
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    };

    // Verify the value was modified
    assert_eq!(map["key1"], 15);
}

#[test]
fn test_and_modify_on_different_occupied_entry() {
    use hashbrown::HashMap;

    // Initialize a hash map and insert a different value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key2", 20);

    // Access the Entry and apply a different modification function
    let entry = map.entry("key2");
    let modified_entry = entry.and_modify(|v| *v *= 2);

    // The modified_entry should still be occupied
    let _occupied_entry = match modified_entry {
        Entry::Occupied(entry) => entry,
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    };

    // Verify the value was modified
    assert_eq!(map["key2"], 40);
}

#[test]
fn test_and_modify_with_repeated_access() {
    use hashbrown::HashMap;

    // Initialize a hash map and insert a value
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("key3", 30);

    // First modification
    let entry = map.entry("key3");
    let modified_entry = entry.and_modify(|v| *v += 10);

    // Ensure it's still occupied
    let _occupied_entry_first = match modified_entry {
        Entry::Occupied(entry) => entry,
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    };

    // Second modification
    let entry = map.entry("key3");
    let modified_entry_second = entry.and_modify(|v| *v += 5);

    // Ensure it's still occupied
    let _occupied_entry_second = match modified_entry_second {
        Entry::Occupied(entry) => entry,
        Entry::Vacant(_) => panic!("Expected occupied entry"),
    };

    // Verify the final value
    assert_eq!(map["key3"], 45);
}

