// Answer 0

#[test]
fn test_add_dfa_state_for_nfa_state_existing_dead() {
    let config = Config::default();
    let nfa = NFA::default();
    let mut builder = InternalBuilder {
        dfa: DFA {
            config: config.clone(),
            nfa,
            table: vec![],
            starts: vec![],
            min_match_id: StateID::new(0),
            classes: ByteClasses([0; 256]),
            stride2: 8,
            quitset: ByteSet::default(),
            cache_capacity: 0,
        },
        uncompiled_nfa_ids: vec![],
        nfa_to_dfa_id: vec![DEAD; 10],
        stack: vec![],
        seen: SparseSet {
            len: 0,
            dense: vec![],
            sparse: vec![],
        },
        matched: false,
        config,
        nfa: &nfa,
        classes: ByteClasses([0; 256]),
    };

    let nfa_id = StateID::new(5);
    let result = builder.add_dfa_state_for_nfa_state(nfa_id);
}

#[test]
fn test_add_dfa_state_for_nfa_state_error_empty_state() {
    let config = Config::default();
    let nfa = NFA::default();
    let mut builder = InternalBuilder {
        dfa: DFA {
            config: config.clone(),
            nfa,
            table: vec![],
            starts: vec![],
            min_match_id: StateID::new(0),
            classes: ByteClasses([0; 256]),
            stride2: 8,
            quitset: ByteSet::default(),
            cache_capacity: 0,
        },
        uncompiled_nfa_ids: vec![],
        nfa_to_dfa_id: vec![DEAD; 10],
        stack: vec![],
        seen: SparseSet {
            len: 0,
            dense: vec![],
            sparse: vec![],
        },
        matched: false,
        config,
        nfa: &nfa,
        classes: ByteClasses([0; 256]),
    };

    builder.dfa.table = vec![Transition(0); Transition::STATE_ID_LIMIT]; // Fill to exceed capacity
    let nfa_id = StateID::new(6);
    let result = builder.add_dfa_state_for_nfa_state(nfa_id);
}

