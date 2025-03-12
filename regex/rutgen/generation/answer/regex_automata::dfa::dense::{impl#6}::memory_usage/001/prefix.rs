// Answer 0

#[test]
fn test_memory_usage_non_empty_tables() {
    let transition_table = TransitionTable {
        table: vec![1, 2, 3, 4],
        classes: ByteClasses::default(),
        stride2: 2,
    };
    let start_table = StartTable {
        table: vec![0, 1, 2, 3],
        kind: StartKind::Both,
        start_map: Default::default(),
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: vec![0, 1],
        pattern_ids: vec![1],
        pattern_len: 1,
    };
    let accels = Accels {
        accels: vec![1, 2],
    };
    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        accels,
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };

    dfa.memory_usage();
}

#[test]
fn test_memory_usage_zero_states() {
    let transition_table = TransitionTable {
        table: Vec::new(),
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let start_table = StartTable {
        table: Vec::new(),
        kind: StartKind::Both,
        start_map: Default::default(),
        stride: 1,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: Vec::new(),
        pattern_ids: Vec::new(),
        pattern_len: 0,
    };
    let accels = Accels {
        accels: Vec::new(),
    };
    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        accels,
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };

    dfa.memory_usage();
}

#[test]
fn test_memory_usage_maximum_states() {
    let transition_table = TransitionTable {
        table: vec![0; 256],
        classes: ByteClasses::default(),
        stride2: 9,
    };
    let start_table = StartTable {
        table: vec![0; 2 * 9],
        kind: StartKind::Both,
        start_map: Default::default(),
        stride: 9,
        pattern_len: Some(256),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: vec![0; 256],
        pattern_ids: vec![1; 256],
        pattern_len: 256,
    };
    let accels = Accels {
        accels: vec![1; 256],
    };
    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        accels,
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };

    dfa.memory_usage();
}

