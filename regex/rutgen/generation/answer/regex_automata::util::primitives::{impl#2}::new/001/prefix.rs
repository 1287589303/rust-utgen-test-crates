// Answer 0

#[test]
fn test_small_index_new_success_0() {
    let index = 0;
    let result = SmallIndex::new(index);
}

#[test]
fn test_small_index_new_success_boundary() {
    let index = SmallIndex::MAX.as_usize();
    let result = SmallIndex::new(index);
}

#[test]
fn test_small_index_new_failure_above_max() {
    let index = SmallIndex::MAX.as_usize() + 1;
    let result = SmallIndex::new(index);
}

#[test]
fn test_small_index_new_failure_large_value() {
    let index = usize::MAX;
    let result = SmallIndex::new(index);
}

