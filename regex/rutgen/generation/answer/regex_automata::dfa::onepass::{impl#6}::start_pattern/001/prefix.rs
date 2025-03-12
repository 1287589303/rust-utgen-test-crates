// Answer 0

#[test]
fn test_start_pattern_valid_case() {
    let mut dfa = DFA {
        config: Config::new().starts_for_each_pattern(true),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![StateID(1), StateID(2), StateID(3)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let pattern_id = PatternID(0);
    let result = dfa.start_pattern(pattern_id);
}

#[test]
fn test_start_pattern_middle_case() {
    let mut dfa = DFA {
        config: Config::new().starts_for_each_pattern(true),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![StateID(1), StateID(2), StateID(3)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let pattern_id = PatternID(1);
    let result = dfa.start_pattern(pattern_id);
}

#[test]
fn test_start_pattern_boundary_case() {
    let mut dfa = DFA {
        config: Config::new().starts_for_each_pattern(true),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![StateID(1), StateID(2), StateID(3)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let pattern_id = PatternID(2); // matches starts.len() - 1
    let result = dfa.start_pattern(pattern_id);
}

#[test]
fn test_start_pattern_out_of_bounds() {
    let mut dfa = DFA {
        config: Config::new().starts_for_each_pattern(true),
        nfa: NFA::default(),
        table: vec![],
        starts: vec![StateID(1), StateID(2), StateID(3)],
        min_match_id: StateID(0),
        classes: ByteClasses([0; 256]),
        alphabet_len: 256,
        stride2: 9,
        pateps_offset: 0,
        explicit_slot_start: 0,
    };

    let pattern_id = PatternID(3); // out of bounds
    let result = dfa.start_pattern(pattern_id);
}

