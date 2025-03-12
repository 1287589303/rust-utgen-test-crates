// Answer 0

#[test]
fn test_shift_insert_valid_index() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, String>::new();
    let hash = HashValue(1);
    let key = 42;
    let value = "Test".to_string();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry { map: ref_mut, hash, key };

    // We can safely insert at index 0
    let result = vacant_entry.shift_insert(0, value);
}

#[test]
fn test_shift_insert_boundary_index_lower() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, String>::new();
    let hash = HashValue(1);
    let key = 42;
    let value = "Test".to_string();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry { map: ref_mut, hash, key };

    // Attempting to insert at index 0 on an empty entries should work
    let result = vacant_entry.shift_insert(0, value);
}

#[test]
#[should_panic]
fn test_shift_insert_invalid_index_above() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, String>::new();
    let hash = HashValue(1);
    let key = 42;
    let value = "Test".to_string();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry { map: ref_mut, hash, key };

    // Attempting to insert at index 1 on an empty entries should panic
    let _result = vacant_entry.shift_insert(1, value);
}

#[test]
fn test_shift_insert_valid_index_with_multiple_entries() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, String>::new();
    let hash_1 = HashValue(1);
    let key_1 = 42;
    let value_1 = "First".to_string();
    let hash_2 = HashValue(2);
    let key_2 = 43;
    let value_2 = "Second".to_string();

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry_1 = VacantEntry { map: ref_mut, hash: hash_1, key: key_1 };
    let _ = vacant_entry_1.shift_insert(0, value_1);

    // Now we can insert the second entry at index 1
    let vacant_entry_2 = VacantEntry { map: ref_mut, hash: hash_2, key: key_2 };
    let result = vacant_entry_2.shift_insert(1, value_2);
} 

#[test]
#[should_panic]
fn test_shift_insert_invalid_index_negative() {
    let mut indices = Indices::new();
    let mut entries = Entries::<usize, String>::new();
    let hash = HashValue(1);
    let key = 42;
    let value = "Test".to_string();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let vacant_entry = VacantEntry { map: ref_mut, hash, key };

    // Attempting to insert at an invalid negative index should panic
    let _result = vacant_entry.shift_insert(usize::MAX, value);
} 

