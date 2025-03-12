// Answer 0

#[test]
fn test_shift_remove_entry_valid_index() {
    struct TestMap {
        indices: Indices,
        entries: Entries<i32, String>,
    }
    
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    entries.push((1, "Value1".to_string()));
    entries.push((2, "Value2".to_string()));
    
    let mut map = TestMap { indices, entries };
    let index = 1;  // Valid index for non-empty map
    
    let mut entry = IndexedEntry::new(&mut map, index);
    entry.shift_remove_entry();
}

#[test]
fn test_shift_remove_entry_first_index() {
    struct TestMap {
        indices: Indices,
        entries: Entries<i32, String>,
    }
    
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    entries.push((1, "Value1".to_string()));
    entries.push((2, "Value2".to_string()));
    
    let mut map = TestMap { indices, entries };
    let index = 0;  // Valid index for non-empty map
    
    let mut entry = IndexedEntry::new(&mut map, index);
    entry.shift_remove_entry();
}

#[test]
fn test_shift_remove_entry_last_index() {
    struct TestMap {
        indices: Indices,
        entries: Entries<i32, String>,
    }
    
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    entries.push((1, "Value1".to_string()));
    entries.push((2, "Value2".to_string()));
    
    let mut map = TestMap { indices, entries };
    let index = 1;  // Valid index for non-empty map
    
    let mut entry = IndexedEntry::new(&mut map, index);
    entry.shift_remove_entry();
}

#[test]
fn test_shift_remove_entry_with_multiple_elements() {
    struct TestMap {
        indices: Indices,
        entries: Entries<i32, String>,
    }
    
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    entries.push((1, "Value1".to_string()));
    entries.push((2, "Value2".to_string()));
    entries.push((3, "Value3".to_string()));
    
    let mut map = TestMap { indices, entries };
    let index = 1;  // Valid index for non-empty map
    
    let mut entry = IndexedEntry::new(&mut map, index);
    entry.shift_remove_entry();
}

