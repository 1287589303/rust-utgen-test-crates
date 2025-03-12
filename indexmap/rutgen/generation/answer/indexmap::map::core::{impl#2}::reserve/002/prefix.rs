// Answer 0

#[test]
fn test_reserve_zero_capacity() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve(0);
}

#[test]
fn test_reserve_empty_entries() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.reserve(0);
}

#[test]
fn test_reserve_capacity_equal_to_remaining_space() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    index_map.reserve(5);
}

#[test]
fn test_reserve_capacity_one_above_remaining_space() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    index_map.reserve(6);
}

