// Answer 0

#[test]
fn test_len_empty_state_set() {
    let state_set = StateSet::empty();
    let _ = state_set.len();
}

#[test]
fn test_len_single_state_id() {
    let mut state_set = StateSet::empty();
    state_set.add(StateID(0.into()));
    let _ = state_set.len();
}

#[test]
fn test_len_multiple_state_ids() {
    let mut state_set = StateSet::empty();
    for id in 0..10 {
        state_set.add(StateID(id.into()));
    }
    let _ = state_set.len();
}

#[test]
fn test_len_max_state_ids() {
    let mut state_set = StateSet::empty();
    for id in 0..1000 {
        state_set.add(StateID(id.into()));
    }
    let _ = state_set.len();
}

#[test]
fn test_len_after_clear() {
    let mut state_set = StateSet::empty();
    state_set.add(StateID(0.into()));
    state_set.clear();
    let _ = state_set.len();
}

