// Answer 0

#[test]
fn test_fmt_no_states_and_empty_start_table() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 0,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Unanchored,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: StateID(0),
            quit_id: StateID(0),
            min_match: StateID(0),
            max_match: StateID(0),
            min_accel: StateID(0),
            max_accel: StateID(0),
            min_start: StateID(0),
            max_start: StateID(0),
        },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };

    let mut output = String::new();
    let _ = fmt::write(&mut output, format_args!("{:?}", dfa));
}

#[test]
fn test_fmt_with_known_flags_and_no_patterns() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 0,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Unanchored,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: StateID(0),
            quit_id: StateID(0),
            min_match: StateID(0),
            max_match: StateID(0),
            min_accel: StateID(0),
            max_accel: StateID(0),
            min_start: StateID(0),
            max_start: StateID(0),
        },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };

    let mut output = String::new();
    let _ = fmt::write(&mut output, format_args!("{:?}", dfa));
}

