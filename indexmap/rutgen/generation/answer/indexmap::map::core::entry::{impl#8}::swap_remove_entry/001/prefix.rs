// Answer 0

#[test]
fn test_swap_remove_entry_valid() {
    struct TestKey;
    struct TestValue;

    let mut indices = Indices::new();
    let mut entries = Entries::<TestKey, TestValue>::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    
    map.insert(TestKey, TestValue);
    
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_multiple() {
    struct TestKey;
    struct TestValue;

    let mut indices = Indices::new();
    let mut entries = Entries::<TestKey, TestValue>::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    
    map.insert(TestKey, TestValue);
    map.insert(TestKey, TestValue); // Added a second entry
    
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_boundary() {
    struct TestKey;
    struct TestValue;

    let mut indices = Indices::new();
    let mut entries = Entries::<TestKey, TestValue>::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    
    map.insert(TestKey, TestValue);
    
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = entry.swap_remove_entry(); // Test removing last entry in a single-entry map
}

