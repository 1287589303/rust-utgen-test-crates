// Answer 0

#[test]
fn test_fmt_valid_entry() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = format!("{:?}", entry);
}

#[test]
fn test_fmt_multiple_entries() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry1 = IndexedEntry::new(&mut map, 0);
    let entry2 = IndexedEntry::new(&mut map, 1);
    let _ = format!("{:?}", entry1);
    let _ = format!("{:?}", entry2);
}

#[test]
fn test_fmt_boundary_entry_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = format!("{:?}", entry);
}

#[test]
#[should_panic]
fn test_fmt_invalid_entry_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let _ = IndexedEntry::new(&mut map, 1);
}

