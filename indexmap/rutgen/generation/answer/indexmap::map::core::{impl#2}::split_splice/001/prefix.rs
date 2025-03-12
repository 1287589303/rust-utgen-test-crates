// Answer 0

#[test]
fn test_split_splice_empty_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = map.split_splice(0..0);
}

#[test]
fn test_split_splice_valid_range_zero() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.insert_full(0, 1, 2);
    let result = map.split_splice(0..1);
}

#[test]
fn test_split_splice_valid_range_full() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    map.insert_full(0, 1, 2);
    map.insert_full(1, 2, 3);
    let result = map.split_splice(0..2);
}

#[test]
fn test_split_splice_partial_split() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    map.insert_full(0, 1, 2);
    map.insert_full(1, 2, 3);
    let result = map.split_splice(1..2);
}

#[test]
#[should_panic]
fn test_split_splice_out_of_bounds() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = map.split_splice(1..2);
}

