// Answer 0

#[test]
fn test_small_index_must_zero() {
    let index = 0;
    let result = SmallIndex::must(index);
}

#[test]
fn test_small_index_must_max() {
    let index = SmallIndex::MAX.as_usize();
    let result = SmallIndex::must(index);
}

#[test]
fn test_small_index_must_valid_middle() {
    let index = SmallIndex::MAX.as_usize() / 2;
    let result = SmallIndex::must(index);
}

