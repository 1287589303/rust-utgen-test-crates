// Answer 0

#[test]
fn test_insert_new_element_with_capacity() {
    let capacity = 10;
    let mut sparse_set = SparseSet::new(capacity);
    let state_id = StateID::new(0); // Assuming StateID can be created this way

    sparse_set.insert(state_id);
}

#[test]
fn test_insert_multiple_elements_with_capacity() {
    let capacity = 10;
    let mut sparse_set = SparseSet::new(capacity);
    let state_id1 = StateID::new(1);
    let state_id2 = StateID::new(2);
    
    sparse_set.insert(state_id1);
    sparse_set.insert(state_id2);
}

#[test]
fn test_insert_at_capacity_limit() {
    let capacity = 5;
    let mut sparse_set = SparseSet::new(capacity);
    for i in 0..capacity - 1 {
        let state_id = StateID::new(i);
        sparse_set.insert(state_id);
    }
    let new_state_id = StateID::new(capacity - 1);
    
    sparse_set.insert(new_state_id);
}

#[test]
fn test_insert_state_ids_below_capacity_limit() {
    let capacity = 3;
    let mut sparse_set = SparseSet::new(capacity);
    let state_id1 = StateID::new(0);
    let state_id2 = StateID::new(1);
    
    sparse_set.insert(state_id1);
    sparse_set.insert(state_id2);
}

