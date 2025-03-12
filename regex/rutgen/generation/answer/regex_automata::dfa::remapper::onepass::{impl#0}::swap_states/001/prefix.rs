// Answer 0

#[test]
fn test_swap_states_valid_ids() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![Transition { start: 0, end: 255, next: StateID(1) }; 512],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.swap_states(StateID(1), StateID(2));
}

#[test]
fn test_swap_states_equal_ids() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![Transition { start: 0, end: 255, next: StateID(1) }; 512],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.swap_states(StateID(1), StateID(1));
}

#[test]
#[should_panic]
fn test_swap_states_exceeding_ids() {
    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![Transition { start: 0, end: 255, next: StateID(1) }; 512],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.swap_states(StateID(511), StateID(512));
}

