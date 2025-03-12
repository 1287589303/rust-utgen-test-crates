// Answer 0

#[test]
fn test_to_ne_bytes_zero() {
    let index = SmallIndex::new_unchecked(0);
    let _bytes = index.to_ne_bytes();
}

#[test]
fn test_to_ne_bytes_max() {
    let index = SmallIndex::new_unchecked(u32::MAX as usize);
    let _bytes = index.to_ne_bytes();
}

#[test]
fn test_to_ne_bytes_median() {
    let index = SmallIndex::new_unchecked((u32::MAX / 2) as usize);
    let _bytes = index.to_ne_bytes();
}

