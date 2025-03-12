// Answer 0

#[test]
fn test_as_usize_zero() {
    let index = SmallIndex::new_unchecked(0);
    let _result = index.as_usize();
}

#[test]
fn test_as_usize_max() {
    let index = SmallIndex::new_unchecked(SmallIndex::MAX.as_usize());
    let _result = index.as_usize();
}

#[test]
fn test_as_usize_mid() {
    let index = SmallIndex::new_unchecked(SmallIndex::MAX.as_usize() / 2);
    let _result = index.as_usize();
} 

#[test]
fn test_as_usize_one_more() {
    let index = SmallIndex::new_unchecked(SmallIndex::MAX.as_usize() - 1);
    let _result = index.as_usize();
}

