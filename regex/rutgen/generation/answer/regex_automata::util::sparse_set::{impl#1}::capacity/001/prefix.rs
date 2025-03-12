// Answer 0

#[test]
fn test_capacity_zero() {
    let sparse_set = SparseSet::new(0);
    let capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_one() {
    let sparse_set = SparseSet::new(1);
    let capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_ten() {
    let sparse_set = SparseSet::new(10);
    let capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_max_state_id() {
    let max_state_id = StateID(u32::MAX);
    let sparse_set = SparseSet::new(max_state_id.0 as usize);
    let capacity = sparse_set.capacity();
}

