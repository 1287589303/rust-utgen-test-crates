// Answer 0

#[test]
fn test_shift_remove_valid_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry = IndexedEntry::new(&mut map, 0);
    let value = entry.shift_remove();
}

#[test]
fn test_shift_remove_last_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let entry = IndexedEntry::new(&mut map, 0);
    let value = entry.shift_remove();
}

#[test]
fn test_shift_remove_middle_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    let entry = IndexedEntry::new(&mut map, 1);
    let value = entry.shift_remove();
}

#[test]
fn test_shift_remove_boundary_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry = IndexedEntry::new(&mut map, 1);
    let value = entry.shift_remove();
}

