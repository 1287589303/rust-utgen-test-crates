// Answer 0

#[test]
fn test_add_nfa_states_byte_range() {
    let nfa = thompson::NFA::new("abc").unwrap();
    let mut set = SparseSet::new(3);
    let state_id_1 = StateID(0);
    let state_id_2 = StateID(1);
    let state_id_3 = StateID(2);
    set.insert(state_id_1);
    set.insert(state_id_2);
    set.insert(state_id_3);
    let mut builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(0),
    };
    builder.set_look_need(|_| LookSet::singleton(Look::Start));

    add_nfa_states(&nfa, &set, &mut builder);
}

#[test]
#[should_panic]
fn test_add_nfa_states_no_look_need() {
    let nfa = thompson::NFA::new("abc").unwrap();
    let mut set = SparseSet::new(1);
    set.insert(StateID(0));
    let mut builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(0),
    };

    add_nfa_states(&nfa, &set, &mut builder);
}

#[test]
fn test_add_nfa_states_not_byte_range() {
    let nfa = thompson::NFA::new("abc").unwrap();
    let mut set = SparseSet::new(1);
    let state_id = StateID(1);
    set.insert(state_id);
    let mut builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(0),
    };
    builder.set_look_need(|_| LookSet::full());

    add_nfa_states(&nfa, &set, &mut builder);
}

