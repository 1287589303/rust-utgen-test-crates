// Answer 0

#[test]
fn test_into_mut_valid_index() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let index = 0;
    let entry = IndexedEntry::new(&mut map, index);
    let value_ref: &mut str = entry.into_mut();
    *value_ref = "new_value1";
}

#[test]
fn test_into_mut_boundary_index_lower() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let index = 0; 
    let entry = IndexedEntry::new(&mut map, index);
    let value_ref: &mut str = entry.into_mut();
    *value_ref = "new_value1";
}

#[test]
fn test_into_mut_boundary_index_upper() {
    let mut map = IndexMapCore::new();
    map.insert("key1", "value1");
    let index = map.entries.len() - 1; 
    let entry = IndexedEntry::new(&mut map, index);
    let value_ref: &mut str = entry.into_mut();
    *value_ref = "new_value1";
}

