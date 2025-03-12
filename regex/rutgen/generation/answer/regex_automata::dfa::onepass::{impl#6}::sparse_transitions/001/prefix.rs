// Answer 0

#[test]
fn test_sparse_transitions_with_valid_state() {
    let stride2 = 8; // 2^3
    let alphabet_len = 256; // maximum alphabet length
    let table = vec![Transition { start: 0, end: 0, next: StateID(0) }; (1 << stride2) * alphabet_len];

    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table,
        starts: vec![StateID(0)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len,
        stride2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let sid = StateID(0);
    let _sparse_iter = dfa.sparse_transitions(sid);
}

#[test]
fn test_sparse_transitions_with_middle_state() {
    let stride2 = 4; // 2^2
    let alphabet_len = 128; // less than maximum
    let table = vec![Transition { start: 0, end: 0, next: StateID(1) }; (1 << stride2) * alphabet_len];

    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table,
        starts: vec![StateID(1)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len,
        stride2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let sid = StateID(1);
    let _sparse_iter = dfa.sparse_transitions(sid);
}

#[test]
fn test_sparse_transitions_with_high_state() {
    let stride2 = 6; // 2^2
    let alphabet_len = 256; // maximum alphabet length
    let table = vec![Transition { start: 0, end: 0, next: StateID(2) }; (1 << stride2) * alphabet_len];

    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table,
        starts: vec![StateID(2)],
        min_match_id: StateID(2),
        classes: ByteClasses([0; 256]),
        alphabet_len,
        stride2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let sid = StateID(2);
    let _sparse_iter = dfa.sparse_transitions(sid);
}

#[test]
fn test_sparse_transitions_with_edge_state_id() {
    let stride2 = 1; // 2^0
    let alphabet_len = 1; // minimum alphabet length
    let table = vec![Transition { start: 0, end: 0, next: StateID(3) }; (1 << stride2) * alphabet_len];

    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table,
        starts: vec![StateID(3)],
        min_match_id: StateID(3),
        classes: ByteClasses([0; 256]),
        alphabet_len,
        stride2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let sid = StateID(3);
    let _sparse_iter = dfa.sparse_transitions(sid);
}

