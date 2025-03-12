// Answer 0

#[test]
#[should_panic]
fn test_resize_capacity_exceeding_limit() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(u32::MAX.as_usize() + 1);
}

#[test]
#[should_panic]
fn test_resize_capacity_exceeding_limit_boundary_case() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(u32::MAX.as_usize() + 2);
}

