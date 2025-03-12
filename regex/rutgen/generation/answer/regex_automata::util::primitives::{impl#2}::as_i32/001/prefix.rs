// Answer 0

#[test]
fn test_as_i32_zero() {
    let index = SmallIndex::new_unchecked(0);
    let result = index.as_i32();
}

#[test]
fn test_as_i32_boundary_min() {
    let index = SmallIndex::new_unchecked(1);
    let result = index.as_i32();
}

#[test]
fn test_as_i32_boundary_max() {
    let index = SmallIndex::new_unchecked(core::i32::MAX as usize);
    let result = index.as_i32();
}

#[test]
fn test_as_i32_boundary_above_max() {
    let index = SmallIndex::new_unchecked(core::i32::MAX as usize - 1);
    let result = index.as_i32();
}

