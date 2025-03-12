// Answer 0

#[test]
fn test_try_reserve_exact_zero_additional() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.try_reserve_exact(0).unwrap();
}

#[test]
fn test_try_reserve_exact_max_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    index_map.try_reserve_exact(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY).unwrap();
}

#[test]
#[should_panic]
fn test_try_reserve_exact_exceed_capacity() {
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    index_map.try_reserve_exact(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY + 1).unwrap_err();
}

