// Answer 0

#[test]
fn test_index_zero() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let indexed_entry = IndexedEntry::new(&mut map, 0);
    indexed_entry.index();
}

#[test]
fn test_index_last() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let indexed_entry = IndexedEntry::new(&mut map, 1);
    indexed_entry.index();
}

#[test]
fn test_index_out_of_bounds_high() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let indexed_entry = IndexedEntry::new(&mut map, 2);
    indexed_entry.index();
}

#[test]
fn test_index_out_of_bounds_low() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let indexed_entry = IndexedEntry::new(&mut map, usize::MAX);
    indexed_entry.index();
}

