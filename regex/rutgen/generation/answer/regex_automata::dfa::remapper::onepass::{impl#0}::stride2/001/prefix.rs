// Answer 0

#[test]
fn test_stride2_empty_dfa() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: Vec::new(),
        starts: Vec::new(),
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.stride2();
}

#[test]
fn test_stride2_single_state_dfa() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![Transition { byte: 0, next: StateID(1) }],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.stride2();
}

#[test]
fn test_stride2_multiple_states_dfa() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![
            Transition { byte: 0, next: StateID(1) },
            Transition { byte: 1, next: StateID(2) },
        ],
        starts: vec![StateID(0)],
        min_match_id: StateID(2),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.stride2();
}

