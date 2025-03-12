// Answer 0

#[test]
fn test_insert_already_contains_id() {
    let mut sparse_set = SparseSet::new(10);
    let state_id = StateID::new_unchecked(2);
    sparse_set.insert(state_id);
    let result = sparse_set.insert(state_id);
}

#[test]
fn test_insert_multiple_same_id() {
    let mut sparse_set = SparseSet::new(5);
    let state_id = StateID::new_unchecked(1);
    sparse_set.insert(state_id);
    let first_insert_result = sparse_set.insert(state_id);
    let second_insert_result = sparse_set.insert(state_id);
}

#[test]
fn test_insert_boundary_case() {
    let mut sparse_set = SparseSet::new(1);
    let state_id = StateID::new_unchecked(0);
    sparse_set.insert(state_id);
    let result = sparse_set.insert(state_id);
}

