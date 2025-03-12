// Answer 0

#[test]
fn test_try_from_zero() {
    let result = SmallIndex::try_from(0);
}

#[test]
fn test_try_from_max() {
    let result = SmallIndex::try_from(SmallIndex::MAX.as_u64());
}

