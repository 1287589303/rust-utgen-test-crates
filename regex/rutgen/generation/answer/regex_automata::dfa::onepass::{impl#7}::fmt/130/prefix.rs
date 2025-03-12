// Answer 0

#[test]
fn test_fmt_empty_dfa() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: Vec::new(),
        starts: Vec::new(),
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let mut output = Vec::new();
    let formatter = &mut output;

    dfa.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_dfa_with_states_only() {
    let state_id_0 = StateID::default();
    let transition = Transition {
        byte: 1,
        next: StateID::default(),
    };
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![transition],
        starts: vec![state_id_0],
        min_match_id: state_id_0,
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 1,
    };

    let mut output = Vec::new();
    let formatter = &mut output;

    dfa.fmt(formatter).unwrap();
}

#[test]
fn test_fmt_dfa_empty_start() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: Vec::new(),
        starts: vec![StateID::default()],
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 0,
        stride2: 0,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let mut output = Vec::new();
    let formatter = &mut output;

    let result = dfa.fmt(formatter);
    assert!(result.is_err());
}

#[test]
fn test_fmt_dfa_single_non_matching_state() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA(Arc::new(Inner::default())),
        table: vec![Transition { byte: 0, next: StateID::default() }],
        starts: Vec::new(),
        min_match_id: StateID::default(),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 1,
    };

    let mut output = Vec::new();
    let formatter = &mut output;

    let result = dfa.fmt(formatter);
    assert!(result.is_ok());
}

