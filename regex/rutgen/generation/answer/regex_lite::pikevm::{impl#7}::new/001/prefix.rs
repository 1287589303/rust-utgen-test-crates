// Answer 0

#[test]
fn test_new_sparse_set_capacity_zero() {
    let set = SparseSet::new(0);
}

#[test]
fn test_new_sparse_set_capacity_one() {
    let set = SparseSet::new(1);
}

#[test]
fn test_new_sparse_set_capacity_two() {
    let set = SparseSet::new(2);
}

#[test]
fn test_new_sparse_set_capacity_max() {
    let set = SparseSet::new(u32::MAX.as_usize());
}

