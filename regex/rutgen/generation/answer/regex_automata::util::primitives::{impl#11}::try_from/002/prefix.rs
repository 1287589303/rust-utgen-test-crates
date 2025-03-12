// Answer 0

#[test]
fn test_try_from_zero() {
    let index: usize = 0;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_max() {
    let index: usize = SmallIndex::MAX.as_usize();
    let result = SmallIndex::try_from(index);
}

