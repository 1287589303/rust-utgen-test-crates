// Answer 0

#[test]
fn test_dfa_empty_states() {
    let mut f = core::fmt::Formatter::new();
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
    dfa.fmt(&mut f).unwrap();
}

#[test]
fn test_dfa_one_start_state() {
    let mut f = core::fmt::Formatter::new();
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { byte: 0, next: StateID::default() }],
        starts: vec![StateID::default()],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.fmt(&mut f).unwrap();
}

#[test]
fn test_dfa_multiple_start_states() {
    let mut f = core::fmt::Formatter::new();
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { byte: 0, next: StateID::default() }, Transition { byte: 1, next: StateID::default() }],
        starts: vec![StateID::default(), StateID::default()],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 2,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.fmt(&mut f).unwrap();
}

#[test]
fn test_dfa_max_state_length() {
    let mut f = core::fmt::Formatter::new();
    let max_len = StateID::max_value().as_usize();
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition { byte: 0, next: StateID::default() }; max_len],
        starts: vec![StateID::must(max_len)],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    dfa.fmt(&mut f).unwrap();
}

