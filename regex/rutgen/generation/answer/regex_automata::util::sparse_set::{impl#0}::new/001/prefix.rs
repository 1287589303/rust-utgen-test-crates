// Answer 0

#[test]
fn test_new_sparse_sets_min_capacity() {
    let capacity = 1; // Minimum valid capacity
    let sparse_sets = SparseSets::new(capacity);
}

#[test]
fn test_new_sparse_sets_max_capacity() {
    let capacity = StateID::LIMIT; // Maximum valid capacity
    let sparse_sets = SparseSets::new(capacity);
}

#[test]
#[should_panic]
fn test_new_sparse_sets_exceeding_capacity() {
    let capacity = StateID::LIMIT + 1; // Exceeding valid capacity
    let sparse_sets = SparseSets::new(capacity);
}

