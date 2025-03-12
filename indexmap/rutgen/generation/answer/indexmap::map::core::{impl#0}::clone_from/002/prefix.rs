// Answer 0

#[test]
fn test_clone_from_empty() {
    let mut self_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let other_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    self_map.clone_from(&other_map);
}

#[test]
fn test_clone_from_with_capacity_equal() {
    let mut self_map = IndexMapCore::with_capacity(5);
    let other_map = IndexMapCore::with_capacity(5);
    self_map.clone_from(&other_map);
}

#[test]
fn test_clone_from_with_non_empty_equal_capacity() {
    let mut self_map = IndexMapCore::with_capacity(5);
    self_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    self_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });
    
    let mut other_map = IndexMapCore::with_capacity(5);
    other_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    other_map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });

    self_map.clone_from(&other_map);
}

#[test]
fn test_clone_from_full_capacity() {
    let mut self_map = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY {
        self_map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }

    let mut other_map = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY {
        other_map.entries.push(Bucket { hash: HashValue::default(), key: i, value: i });
    }

    self_map.clone_from(&other_map);
}

