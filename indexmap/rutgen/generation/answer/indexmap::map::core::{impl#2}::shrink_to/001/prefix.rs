// Answer 0

#[test]
fn test_shrink_to_zero_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.shrink_to(0);
}

#[test]
fn test_shrink_to_one_entry() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        // Simulating adding entries
        map.push_entry(i as HashValue, i, i * 10);
    }
    map.shrink_to(1);
}

#[test]
fn test_shrink_to_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::MAX_ENTRIES_CAPACITY {
        // Simulating adding entries
        map.push_entry(i as HashValue, i, i * 10);
    }
    map.shrink_to(IndexMapCore::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_shrink_to_exceeding_current_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.shrink_to(10);
}

#[test]
fn test_shrink_to_capacity_half() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    for i in 0..10 {
        // Simulating adding entries
        map.push_entry(i as HashValue, i, i * 10);
    }
    map.shrink_to(5);
}

