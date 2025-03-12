// Answer 0

#[test]
fn test_as_u32_min_value() {
    let index = SmallIndex::new_unchecked(0);
    let result = index.as_u32();
}

#[test]
fn test_as_u32_boundary_value() {
    let index = SmallIndex::new_unchecked(SmallIndex::MAX.as_usize() - 1);
    let result = index.as_u32();
}

#[test]
fn test_as_u32_max_valid() {
    let usize_max = core::usize::MAX;
    let index = SmallIndex::new_unchecked(if usize_max < SmallIndex::LIMIT { usize_max } else { SmallIndex::MAX.as_usize() - 1 });
    let result = index.as_u32();
}

