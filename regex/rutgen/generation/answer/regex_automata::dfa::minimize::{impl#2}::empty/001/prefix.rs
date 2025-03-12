// Answer 0

#[test]
fn test_empty_state_set() {
    let state_set = StateSet::empty();
}

#[test]
fn test_empty_state_set_id_vector() {
    let state_set = StateSet::empty();
    let ids = state_set.ids.borrow();
    assert!(ids.is_empty());
}

