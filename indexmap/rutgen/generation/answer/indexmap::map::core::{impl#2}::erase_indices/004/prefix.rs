// Answer 0

#[test]
fn test_erase_indices_degenerate_case() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.indices.insert_unique(1, 0, |_| unreachable!()); // total_indices = 1
    // start = 0, end = 1, erased = 0
    map.erase_indices(0, 1);
}

#[test]
fn test_erase_indices_few_kept_indices() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    map.indices.insert_unique(1, 0, |_| unreachable!()); // total_indices = 1
    map.indices.insert_unique(2, 1, |_| unreachable!());
    map.indices.insert_unique(3, 2, |_| unreachable!()); // total_indices = 3
    // start = 0, end = 1, erased = 0, shifting one index
    map.erase_indices(0, 1);
}

#[test]
fn test_erase_indices_few_adjustments() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    map.indices.insert_unique(1, 0, |_| unreachable!()); // total_indices = 1
    map.indices.insert_unique(2, 1, |_| unreachable!());
    // start = 0, end = 2, erased = 0, shifted = 0
    map.erase_indices(0, 2);
}

