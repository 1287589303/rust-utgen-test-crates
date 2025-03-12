// Answer 0

#[test]
fn test_state_len_non_empty_transition_table() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition(0)],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _ = dfa.state_len();
}

#[test]
fn test_state_len_empty_transition_table() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 1,
        stride2: 1,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _ = dfa.state_len();
}

#[test]
fn test_state_len_multiple_transitions() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition(0), Transition(1), Transition(2)],
        starts: vec![StateID(0)],
        min_match_id: StateID(1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 3,
        stride2: 2,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _ = dfa.state_len();
}

#[test]
fn test_state_len_boundary_state_id() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::default(),
        table: vec![Transition(0)],
        starts: vec![StateID(u32::MAX)],
        min_match_id: StateID(u32::MAX - 1),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 8,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };
    let _ = dfa.state_len();
}

