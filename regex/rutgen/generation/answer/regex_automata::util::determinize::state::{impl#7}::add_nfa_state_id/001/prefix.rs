// Answer 0

#[test]
fn test_add_nfa_state_id_with_minimum_state_id() {
    let mut state_builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(SmallIndex::new(0)),
    };
    let min_state_id = StateID(SmallIndex::new(0));
    state_builder.add_nfa_state_id(min_state_id);
}

#[test]
fn test_add_nfa_state_id_with_maximum_state_id() {
    let mut state_builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(SmallIndex::new(0)),
    };
    let max_state_id = StateID(SmallIndex::new(u32::MAX));
    state_builder.add_nfa_state_id(max_state_id);
}

#[test]
fn test_add_nfa_state_id_with_repeated_state_id() {
    let mut state_builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(SmallIndex::new(0)),
    };
    let repeated_state_id = StateID(SmallIndex::new(5));
    state_builder.add_nfa_state_id(repeated_state_id);
    state_builder.add_nfa_state_id(repeated_state_id);
}

#[test]
fn test_add_nfa_state_id_with_incrementing_state_ids() {
    let mut state_builder = StateBuilderNFA {
        repr: Vec::new(),
        prev_nfa_state_id: StateID(SmallIndex::new(0)),
    };
    for i in 0..10 {
        let state_id = StateID(SmallIndex::new(i));
        state_builder.add_nfa_state_id(state_id);
    }
}

