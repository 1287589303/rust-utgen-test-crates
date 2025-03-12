// Answer 0

#[test]
fn test_insert_new_state_id() {
    let mut sparse_set = SparseSet::new(5);
    let state_id = StateID(0);
    assert!(sparse_set.insert(state_id));
}

#[test]
fn test_insert_multiple_state_ids() {
    let mut sparse_set = SparseSet::new(5);
    let state_id1 = StateID(1);
    let state_id2 = StateID(2);
    assert!(sparse_set.insert(state_id1));
    assert!(sparse_set.insert(state_id2));
}

#[test]
fn test_insert_at_capacity() {
    let mut sparse_set = SparseSet::new(2);
    let state_id1 = StateID(0);
    let state_id2 = StateID(1);
    assert!(sparse_set.insert(state_id1));
    assert!(sparse_set.insert(state_id2));
}

#[test]
fn test_insert_with_different_state_ids() {
    let mut sparse_set = SparseSet::new(3);
    let state_id1 = StateID(3);
    let state_id2 = StateID(4);
    assert!(sparse_set.insert(state_id1));
    assert!(sparse_set.insert(state_id2));
}

