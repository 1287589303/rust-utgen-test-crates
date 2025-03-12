// Answer 0

#[test]
fn test_state_len_empty_table() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.state_len();
}

#[test]
fn test_state_len_with_one_state() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { range: Utf8Range::default(), next_id: StateID::default() }],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.state_len();
}

#[test]
fn test_state_len_with_two_states() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![
            Transition { range: Utf8Range::default(), next_id: StateID::default() },
            Transition { range: Utf8Range::default(), next_id: StateID::default() }
        ],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.state_len();
}

#[test]
fn test_state_len_boundary_case() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { range: Utf8Range::default(), next_id: StateID::default() }; 512],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.state_len();
}

#[test]
fn test_state_len_full_stride() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { range: Utf8Range::default(), next_id: StateID::default() }; 256],
        starts: vec![],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 8,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.state_len();
}

