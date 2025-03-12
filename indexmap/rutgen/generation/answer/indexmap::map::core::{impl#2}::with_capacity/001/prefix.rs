// Answer 0

#[test]
fn test_with_capacity_zero() {
    let result = IndexMapCore::<usize, usize>::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let result = IndexMapCore::<usize, usize>::with_capacity(1);
}

#[test]
fn test_with_capacity_max() {
    let result = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_with_capacity_mid() {
    let result = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY / 2);
}

