// Answer 0

#[test]
fn test_key_mut_valid_index() {
    let mut entries = Entries::<i32, i32>::new();
    entries.push(Bucket { hash: HashValue::from(1), key: 10, value: 20 });
    entries.push(Bucket { hash: HashValue::from(2), key: 30, value: 40 });
    
    let mut indices = Indices::new();
    let mut map = IndexMapCore { entries, indices };
    let index = 0; // Valid index within range

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let key_mut = indexed_entry.key_mut();
}

#[test]
#[should_panic]
fn test_key_mut_invalid_index() {
    let mut entries = Entries::<i32, i32>::new();
    entries.push(Bucket { hash: HashValue::from(1), key: 10, value: 20 });
    
    let mut indices = Indices::new();
    let mut map = IndexMapCore { entries, indices };
    let index = 1; // Invalid index out of range

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let key_mut = indexed_entry.key_mut();
}

#[test]
fn test_key_mut_multiple_entries() {
    let mut entries = Entries::<i32, i32>::new();
    entries.push(Bucket { hash: HashValue::from(1), key: 10, value: 20 });
    entries.push(Bucket { hash: HashValue::from(2), key: 30, value: 40 });
    entries.push(Bucket { hash: HashValue::from(3), key: 50, value: 60 });
    
    let mut indices = Indices::new();
    let mut map = IndexMapCore { entries, indices };
    let index = 2; // Valid index

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let key_mut = indexed_entry.key_mut();
}

#[test]
fn test_key_mut_empty_entries() {
    let mut entries = Entries::<i32, i32>::new();
    
    let mut indices = Indices::new();
    let mut map = IndexMapCore { entries, indices };
    let index = 0; // Index is invalid because entries are empty

    let mut indexed_entry = IndexedEntry::new(&mut map, index);
    let key_mut = indexed_entry.key_mut();
}

