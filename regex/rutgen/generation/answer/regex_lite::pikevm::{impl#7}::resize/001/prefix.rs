// Answer 0

#[test]
fn test_resize_to_zero_capacity() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(0);
}

#[test]
fn test_resize_to_max_capacity() {
    let max_capacity = u32::MAX.as_usize();
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(max_capacity);
}

