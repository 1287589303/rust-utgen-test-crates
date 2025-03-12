// Answer 0

#[test]
fn test_try_reserve_with_zero_additional() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = map.try_reserve(0);
}

#[test]
fn test_try_reserve_with_min_additional_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(0);
    let additional = 1;
    let result = map.try_reserve(additional);
}

#[test]
fn test_try_reserve_with_exceeding_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(0);
    let additional = IndexMapCore::MAX_ENTRIES_CAPACITY - map.indices.capacity();
    let result = map.try_reserve(additional);
}

#[test]
fn test_try_reserve_with_full_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    let additional = 1;
    let result = map.try_reserve(additional);
}

