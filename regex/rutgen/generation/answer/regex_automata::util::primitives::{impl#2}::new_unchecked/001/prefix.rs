// Answer 0

#[test]
fn test_small_index_new_unchecked_zero() {
    let index = 0usize;
    let result = SmallIndex::new_unchecked(index);
}

#[test]
fn test_small_index_new_unchecked_mid_range() {
    let index = (u32::MAX / 2) as usize;
    let result = SmallIndex::new_unchecked(index);
}

#[test]
fn test_small_index_new_unchecked_max() {
    let index = u32::MAX as usize;
    let result = SmallIndex::new_unchecked(index);
}

