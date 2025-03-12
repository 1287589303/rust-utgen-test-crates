// Answer 0

#[test]
fn test_start_with_single_state() {
    let state_id = StateID(0);
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![],
        starts: vec![state_id],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _result = dfa.start();
}

#[test]
fn test_start_with_multiple_states() {
    let state_id1 = StateID(0);
    let state_id2 = StateID(1);
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![],
        starts: vec![state_id1, state_id2],
        min_match_id: StateID(2),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _result = dfa.start();
}

#[test]
fn test_start_with_boundary_state_id() {
    let state_id = StateID(u64::MAX);
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![],
        starts: vec![state_id],
        min_match_id: StateID(u64::MAX - 1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _result = dfa.start();
}

