// Answer 0

#[test]
fn test_swap_states_valid_ids() {
    let mut dfa = OwnedDFA::default();
    let id1 = StateID(0);
    let id2 = StateID(1);
    dfa.add_empty_state().unwrap();
    dfa.add_empty_state().unwrap();
    dfa.swap_states(id1, id2);
}

#[test]
fn test_swap_states_boundary_ids() {
    let mut dfa = OwnedDFA::default();
    let id1 = StateID(0);
    let id2 = StateID(1);
    dfa.add_empty_state().unwrap();
    dfa.add_empty_state().unwrap();
    dfa.swap_states(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_states_same_ids() {
    let mut dfa = OwnedDFA::default();
    let id = StateID(0);
    dfa.add_empty_state().unwrap();
    dfa.swap_states(id, id);
}

#[test]
#[should_panic]
fn test_swap_states_invalid_ids() {
    let mut dfa = OwnedDFA::default();
    let id1 = StateID(0);
    let id2 = StateID(5); // Assuming only one state exists
    dfa.add_empty_state().unwrap();
    dfa.swap_states(id1, id2);
}

