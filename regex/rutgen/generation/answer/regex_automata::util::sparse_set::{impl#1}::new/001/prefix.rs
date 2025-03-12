// Answer 0

#[test]
fn test_sparse_set_new_zero_capacity() {
    let capacity = 0;
    let set = SparseSet::new(capacity);
}

#[test]
fn test_sparse_set_new_small_capacity() {
    let capacity = 1; // Minimum capacity greater than zero
    let set = SparseSet::new(capacity);
}

#[test]
fn test_sparse_set_new_mid_capacity() {
    let capacity = StateID::LIMIT / 2; // Mid range capacity
    let set = SparseSet::new(capacity);
}

#[test]
fn test_sparse_set_new_max_capacity() {
    let capacity = StateID::LIMIT; // Maximum capacity
    let set = SparseSet::new(capacity);
}

#[should_panic]
fn test_sparse_set_new_exceeding_capacity() {
    let capacity = StateID::LIMIT + 1; // Exceeding the limit
    let set = SparseSet::new(capacity);
}

