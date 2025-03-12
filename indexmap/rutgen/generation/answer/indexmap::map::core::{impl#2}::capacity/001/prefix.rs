// Answer 0

#[test]
fn test_capacity_empty() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let _ = map.capacity();
}

#[test]
fn test_capacity_partial() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    let hash_value = HashValue::new(0); // Assuming a method to create a HashValue
    map.push_entry(hash_value, 1, 10);
    let _ = map.capacity();
}

#[test]
fn test_capacity_full() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY {
        let hash_value = HashValue::new(i as u64); // Assuming a method to create a HashValue
        map.push_entry(hash_value, i, i * 10);
    }
    let _ = map.capacity();
}

#[test]
fn test_capacity_exceeding() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    for i in 0..(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY + 1) {
        let hash_value = HashValue::new(i as u64); // Assuming a method to create a HashValue
        map.push_entry(hash_value, i, i * 10);
    }
    let _ = map.capacity();
}

