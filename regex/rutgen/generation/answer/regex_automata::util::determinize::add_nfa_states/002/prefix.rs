// Answer 0

#[test]
fn test_add_nfa_states_with_match_state() {
    let nfa_id = StateID(1);
    let mut builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(0),
    };
    let mut set = SparseSet::new(10);
    set.insert(nfa_id);

    let nfa = NFA::always_match(); // Using a simple NFA that ensures a valid match state

    add_nfa_states(&nfa, &set, &mut builder); // Testing the add_nfa_states function
}

#[test]
fn test_add_nfa_states_with_non_empty_look_need() {
    let nfa_id = StateID(2);
    let mut builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(0),
    };
    let mut set = SparseSet::new(10);
    set.insert(nfa_id);

    let mut look_set = LookSet::full(); // Non-empty look set
    builder.set_look_need(|_| look_set);  

    let nfa = NFA::always_match(); 

    add_nfa_states(&nfa, &set, &mut builder);
}

#[test]
fn test_add_nfa_states_with_multiple_match_states() {
    let nfa_id1 = StateID(3);
    let nfa_id2 = StateID(4);
    let mut builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(0),
    };
    let mut set = SparseSet::new(10);
    set.insert(nfa_id1);
    set.insert(nfa_id2);

    let mut look_set = LookSet::singleton(Look::Match); // Non-empty look set
    builder.set_look_need(|_| look_set);

    let nfa = NFA::always_match(); 

    add_nfa_states(&nfa, &set, &mut builder);
}

