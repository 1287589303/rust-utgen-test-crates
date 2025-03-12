// Answer 0

#[test]
fn test_contains_with_valid_id() {
    let mut sparse_set = SparseSet::new(5);
    let id = StateID::new(1); // Assume StateID can be created this way.
    sparse_set.insert(id.clone());
    assert!(sparse_set.contains(id));
}

#[test]
fn test_contains_with_edge_case_id() {
    let mut sparse_set = SparseSet::new(3);
    let id = StateID::new(2); // This should be within the valid range.
    sparse_set.insert(id.clone());
    assert!(sparse_set.contains(id));
}

#[test]
fn test_contains_when_empty() {
    let sparse_set = SparseSet::new(0);
    let id = StateID::new(0);
    assert!(!sparse_set.contains(id)); // Should return false since set is empty.
}

#[test]
fn test_contains_with_multiple_inserts() {
    let mut sparse_set = SparseSet::new(5);
    let id1 = StateID::new(1);
    let id2 = StateID::new(3);
    sparse_set.insert(id1.clone());
    sparse_set.insert(id2.clone());
    assert!(sparse_set.contains(id1));
    assert!(sparse_set.contains(id2));
}

#[test]
fn test_contains_with_non_existent_id() {
    let mut sparse_set = SparseSet::new(5);
    let id1 = StateID::new(1);
    let id2 = StateID::new(4); // id that will not be inserted
    sparse_set.insert(id1.clone());
    assert!(!sparse_set.contains(id2)); // Should return false for non-existent ID.
}

