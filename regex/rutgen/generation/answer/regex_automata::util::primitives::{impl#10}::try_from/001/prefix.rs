// Answer 0

#[test]
fn test_try_from_exceeding_maximum_by_one() {
    let index: u64 = SmallIndex::MAX.as_u64() + 1;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_exceeding_maximum_by_a_large_value() {
    let index: u64 = SmallIndex::MAX.as_u64() + 1000;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_u64_max() {
    let index: u64 = u64::MAX;
    let result = SmallIndex::try_from(index);
}

