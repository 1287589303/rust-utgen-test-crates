// Answer 0

#[test]
fn test_as_u64_zero() {
    let index = SmallIndex::new_unchecked(0);
    let _result = index.as_u64();
}

#[test]
fn test_as_u64_minimum() {
    let index = SmallIndex::new_unchecked(1);
    let _result = index.as_u64();
}

#[test]
fn test_as_u64_middle() {
    let index = SmallIndex::new_unchecked(2_147_483_647);
    let _result = index.as_u64();
}

#[test]
fn test_as_u64_maximum() {
    let index = SmallIndex::new_unchecked(4_294_967_295);
    let _result = index.as_u64();
}

