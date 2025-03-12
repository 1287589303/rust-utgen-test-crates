// Answer 0

#[test]
fn test_next_empty() {
    let state_ids: Vec<StateID> = Vec::new();
    let mut iter = SparseSetIter(state_ids.iter());
    let result = iter.next();
}

#[test]
fn test_next_single_element() {
    let state_id = StateID::default();
    let state_ids = vec![state_id];
    let mut iter = SparseSetIter(state_ids.iter());
    let result = iter.next();
}

#[test]
fn test_next_multiple_elements() {
    let state_id1 = StateID::default();
    let state_id2 = StateID(SmallIndex(1));
    let state_ids = vec![state_id1, state_id2];
    let mut iter = SparseSetIter(state_ids.iter());
    
    let result1 = iter.next();
    let result2 = iter.next();
    let result3 = iter.next(); // Should return None
}

#[test]
fn test_next_after_all_elements() {
    let state_id1 = StateID::default();
    let state_id2 = StateID(SmallIndex(1));
    let state_ids = vec![state_id1, state_id2];
    let mut iter = SparseSetIter(state_ids.iter());
    
    let _ = iter.next();
    let _ = iter.next();
    let result = iter.next(); // Should be None
}

