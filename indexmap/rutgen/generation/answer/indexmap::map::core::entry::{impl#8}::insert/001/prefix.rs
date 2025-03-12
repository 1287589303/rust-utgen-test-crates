// Answer 0

#[test]
fn test_insert_with_valid_inputs() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert("value2");
}

#[test]
fn test_insert_with_boundary_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert("value2");

    let mut entry_boundary = IndexedEntry::new(&mut map, 0);
    let old_value_boundary = entry_boundary.insert("value3");
}

#[test]
fn test_insert_with_multiple_entries() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let mut entry = IndexedEntry::new(&mut map, 1);
    let old_value = entry.insert("value3");
}

#[test]
fn test_insert_with_empty_map() {
    let mut map = IndexMapCore::new();
    let mut entry = IndexedEntry::new(&mut map, 0);
    let old_value = entry.insert("value4");
}

#[test]
#[should_panic]
fn test_insert_with_invalid_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let mut entry = IndexedEntry::new(&mut map, 1);
    let old_value = entry.insert("value5");
}

