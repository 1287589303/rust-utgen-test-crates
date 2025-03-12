// Answer 0

#[test]
fn test_fmt_with_no_states_and_start_states() {
    let transitions = Transitions {
        sparse: vec![],
        classes: ByteClasses::default(),
        state_len: 0, // no states
        pattern_len: 1, // at least one pattern
    };

    let start_table = StartTable {
        table: vec![1, 2, 3, 4, 5, 6, 7, 8], // more than zero start states
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 4,
        pattern_len: Some(1), // at least one pattern 
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let special = Special {
        max: 0,
        quit_id: 0,
        min_match: 1,
        max_match: 2,
        min_accel: 3,
        max_accel: 4,
        min_start: 5,
        max_start: 6,
    };

    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };

    let _ = fmt::format(format_args!("{:?}", dfa));
}

#[test]
fn test_fmt_with_states_and_patterns() {
    let transitions = Transitions {
        sparse: vec![0, 1, 2, 3],
        classes: ByteClasses::default(),
        state_len: 1, // one state
        pattern_len: 1, // at least one pattern
    };

    let start_table = StartTable {
        table: vec![1, 2, 3, 4, 5, 6, 7, 8], // more than zero start states
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 2,
        pattern_len: Some(1), // at least one pattern 
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let special = Special {
        max: 1,
        quit_id: 1,
        min_match: 1,
        max_match: 1,
        min_accel: 1,
        max_accel: 1,
        min_start: 1,
        max_start: 1,
    };

    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };

    let _ = fmt::format(format_args!("{:?}", dfa));
}

