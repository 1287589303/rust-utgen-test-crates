// Answer 0

#[test]
fn test_try_from_exceeding_max_value_1() {
    let index: u32 = SmallIndex::MAX.as_u32() + 1;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_exceeding_max_value_2() {
    let index: u32 = SmallIndex::MAX.as_u32() + 1000;
    let result = SmallIndex::try_from(index);
}

#[test]
fn test_try_from_exceeding_max_value_3() {
    let index: u32 = u32::MAX;
    let result = SmallIndex::try_from(index);
}

