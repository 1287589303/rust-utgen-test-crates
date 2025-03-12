// Answer 0

#[test]
fn test_len_empty() {
    let map: super::IndexMapCore<usize, usize> = super::IndexMapCore::new();
    let length = map.len();
}

#[test]
fn test_len_with_capacity_zero() {
    let mut map: super::IndexMapCore<usize, usize> = super::IndexMapCore::with_capacity(0);
    let length = map.len();
}

#[test]
fn test_len_with_capacity_max() {
    let mut map: super::IndexMapCore<usize, usize> = super::IndexMapCore::with_capacity(super::IndexMapCore::MAX_ENTRIES_CAPACITY);
    let length = map.len();
}

#[test]
fn test_len_after_insertion() {
    let mut map: super::IndexMapCore<usize, usize> = super::IndexMapCore::new();
    // Simulate insertion by manipulating the indices directly (not implemented here)
    // Assume some insertions increase the length
    let length = map.len(); // This should reflect the number of items added
}

#[test]
fn test_len_after_clear() {
    let mut map: super::IndexMapCore<usize, usize> = super::IndexMapCore::new();
    // Simulate insertion by manipulating the indices directly (not implemented here)
    // Assume some insertions increase the length
    map.clear();
    let length = map.len();
}

