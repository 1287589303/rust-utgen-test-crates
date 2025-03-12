// Answer 0

#[test]
fn test_set_look_have_empty() {
    let mut state_builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(Default::default()),
    };
    state_builder.set_look_have(|look_set| look_set);
}

#[test]
fn test_set_look_have_non_empty() {
    let mut state_builder = StateBuilderNFA {
        repr: vec![1, 2, 3],
        prev_nfa_state_id: StateID(Default::default()),
    };
    state_builder.set_look_have(|look_set| look_set);
}

#[test]
fn test_set_look_have_with_boundary_lookset() {
    let mut state_builder = StateBuilderNFA {
        repr: vec![0, 255],
        prev_nfa_state_id: StateID(Default::default()),
    };
    state_builder.set_look_have(|look_set| look_set);
}

