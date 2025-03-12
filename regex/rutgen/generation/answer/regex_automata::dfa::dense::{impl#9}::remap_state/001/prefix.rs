// Answer 0

#[test]
fn test_remap_state_valid_identifier() {
    let mut dfa = OwnedDFA::default();
    let id = StateID(0.into());
    dfa.add_empty_state().unwrap();
    dfa.remap_state(id, |state| StateID(state.0 + 1));
}

#[test]
fn test_remap_state_boundary_min() {
    let mut dfa = OwnedDFA::default();
    let id = StateID(0.into());
    dfa.add_empty_state().unwrap();
    dfa.remap_state(id, |state| StateID(state.0 + 1));
}

#[test]
fn test_remap_state_boundary_max() {
    let mut dfa = OwnedDFA::default();
    let max_id = StateID(usize::MAX.into());
    dfa.add_empty_state().unwrap();
    dfa.remap_state(max_id, |state| StateID(state.0.wrapping_sub(1)));
}

#[test]
#[should_panic]
fn test_remap_state_invalid_identifier() {
    let mut dfa = OwnedDFA::default();
    let invalid_id = StateID(usize::MAX.into());
    dfa.remap_state(invalid_id, |state| StateID(state.0 + 1));
}

