// Answer 0

#[test]
fn test_insert_new_id_below_capacity() {
    let mut set = SparseSet::new(5);
    let state_id = StateID::new_unchecked(0);
    set.insert(state_id);
}

#[test]
fn test_insert_multiple_ids_below_capacity() {
    let mut set = SparseSet::new(5);
    for i in 0..4 {
        let state_id = StateID::new_unchecked(i);
        set.insert(state_id);
    }
}

#[test]
#[should_panic]
fn test_insert_exceeding_capacity() {
    let mut set = SparseSet::new(5);
    for i in 0..5 {
        let state_id = StateID::new_unchecked(i);
        set.insert(state_id);
    }
    let overflow_id = StateID::new_unchecked(5);
    set.insert(overflow_id);
}

#[test]
fn test_insert_id_at_capacity() {
    let mut set = SparseSet::new(5);
    for i in 0..5 {
        let state_id = StateID::new_unchecked(i);
        set.insert(state_id);
    }
    let id_at_capacity = StateID::new_unchecked(5);
    set.insert(id_at_capacity);
}

