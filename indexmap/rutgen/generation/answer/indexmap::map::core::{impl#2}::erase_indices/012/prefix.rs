// Answer 0

#[test]
fn test_erase_indices_case_erased_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_capacity(2);
    map.push_entry(HashValue(1), 1, 10);
    map.push_entry(HashValue(2), 2, 20);

    map.erase_indices(0, 1);
}

#[test]
fn test_erase_indices_case_start_equals_erased() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_capacity(2);
    map.push_entry(HashValue(1), 1, 10);
    map.push_entry(HashValue(2), 2, 20);

    map.erase_indices(1, 1);
}

#[test]
fn test_erase_indices_case_erased_one_shifted_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_capacity(2);
    map.push_entry(HashValue(1), 1, 10);
    map.push_entry(HashValue(2), 2, 20);
    
    map.erase_indices(0, 2);
}

#[test]
fn test_erase_indices_case_erased_zero_shifted_false() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_capacity(2);
    map.push_entry(HashValue(1), 1, 10);
    map.push_entry(HashValue(2), 2, 20);
    
    map.entries.push(Bucket { hash: HashValue(3), key: 3, value: 30 });

    map.erase_indices(0, 1);
}

#[test]
fn test_erase_indices_case_new_old_entry_false() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.with_capacity(3);
    map.push_entry(HashValue(1), 1, 10);
    map.push_entry(HashValue(2), 2, 20);
    
    map.erase_indices(0, 1);
    
    map.indices.push(HashValue(4));
}

