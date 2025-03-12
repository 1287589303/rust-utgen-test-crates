// Answer 0

#[test]
fn test_try_from_value_above_max() {
    let index = SmallIndex::MAX.as_usize() + 1;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_value_above_max_plus_one() {
    let index = SmallIndex::MAX.as_usize() + 2;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_value_above_max_boundary() {
    let index = SmallIndex::MAX.as_usize() + 10;
    let result = SmallIndex::try_from(index);
}

