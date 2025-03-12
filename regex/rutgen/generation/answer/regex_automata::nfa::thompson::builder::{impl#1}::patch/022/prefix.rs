// Answer 0

#[test]
#[should_panic]
fn test_patch_sparse_state_panic() {
    let mut builder = Builder::new();
    let sparse_state_id = builder.add_sparse(vec![Transition { start: 0, end: 1, next: StateID(0) }]).unwrap();
    let another_state_id = builder.add_empty().unwrap(); // Just to have another valid StateID

    builder.patch(sparse_state_id, another_state_id).unwrap();
}

#[test]
#[should_panic]
fn test_patch_multiple_sparse_states_panic() {
    let mut builder = Builder::new();
    let sparse_state_id_1 = builder.add_sparse(vec![Transition { start: 2, end: 3, next: StateID(1) }]).unwrap();
    let sparse_state_id_2 = builder.add_sparse(vec![Transition { start: 4, end: 5, next: StateID(2) }]).unwrap();
    let another_state_id = builder.add_empty().unwrap(); // Another state to patch to

    builder.patch(sparse_state_id_1, another_state_id).unwrap();
    builder.patch(sparse_state_id_2, another_state_id).unwrap();
}

#[test]
#[should_panic]
fn test_patch_when_sparsity_exists_panic() {
    let mut builder = Builder::new();
    let initial_state_id = builder.add_match().unwrap(); // Create a match state
    let sparse_state_id = builder.add_sparse(vec![Transition { start: 0, end: 255, next: StateID(0) }]).unwrap();

    builder.patch(sparse_state_id, initial_state_id).unwrap(); // This should panic
}

