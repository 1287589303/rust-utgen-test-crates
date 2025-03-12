// Answer 0

#[test]
fn test_contains_index_out_of_bounds() {
    let mut sparse_set = SparseSet::new(1);
    let state_id = StateID(0); // StateID equal to current length of SparseSet
    // Insert a different StateID to ensure one exists
    sparse_set.insert(StateID(1)); 
    let result = sparse_set.contains(state_id);
}

#[test]
fn test_contains_nonexistent_state_id() {
    let mut sparse_set = SparseSet::new(2);
    let state_id = StateID(3); // StateID does not exist in the dense vector
    sparse_set.insert(StateID(0)); // Insert a StateID to the set
    sparse_set.insert(StateID(1)); 
    let result = sparse_set.contains(state_id);
}

