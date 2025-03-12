// Answer 0

#[test]
fn test_reserve_exact_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.reserve_exact(0);
}

#[test]
fn test_reserve_exact_min_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(1);
    map.reserve_exact(1);
}

#[test]
fn test_reserve_exact_boundary() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    map.reserve_exact(5);
}

#[test]
fn test_reserve_exact_max_capacity() {
    let max_capacity = IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY;
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(max_capacity);
    map.reserve_exact(max_capacity);
}

#[test]
#[should_panic]
fn test_reserve_exact_exceeds_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    map.reserve_exact(6);
}

