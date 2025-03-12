// Answer 0

#[test]
fn test_contains_index_at_length() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.len = 1; // Set length to more than 0
    sparse_set.dense.push(StateID(0)); // Add a StateID
    sparse_set.sparse.push(StateID(0)); // Sparse mapping for the ID

    let id = StateID(0); // This ID should exist in the set
    let result = sparse_set.contains(id);
}

#[test]
fn test_contains_index_just_out_of_bounds() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.len = 1; // Set length to more than 0
    sparse_set.dense.push(StateID(0)); // Add a StateID
    sparse_set.sparse.push(StateID(1)); // Sparse mapping for an out-of-bounds index

    let id = StateID(1); // This ID should not exist in the set
    let result = sparse_set.contains(id);
}

#[test]
fn test_contains_with_exact_boundary() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.len = 1; // Set length to more than 0
    sparse_set.dense.push(StateID(0)); // Add a StateID
    sparse_set.sparse.push(StateID(1)); // Sparse mapping for an ID just above current length

    let id = StateID(1); // ID that is equal to len
    let result = sparse_set.contains(id);
}

