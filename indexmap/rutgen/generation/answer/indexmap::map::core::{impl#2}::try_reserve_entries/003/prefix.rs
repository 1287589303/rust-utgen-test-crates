// Answer 0

#[test]
fn test_try_reserve_entries_equal_case_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(0);
    let additional = 0;
    map.try_reserve_entries(additional);
}

#[test]
fn test_try_reserve_entries_equal_case_one() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    let additional = 1;
    map.try_reserve_entries(additional);
}

#[test]
fn test_try_reserve_entries_equal_case_max_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    let additional = IndexMapCore::MAX_ENTRIES_CAPACITY;
    map.try_reserve_entries(additional);
}

