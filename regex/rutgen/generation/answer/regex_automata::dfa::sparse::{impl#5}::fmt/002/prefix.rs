// Answer 0

#[test]
fn test_fmt_output_ok_some() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![0u8; 10],
            classes: ByteClasses::default(),
            state_len: 1,
            pattern_len: 1,
        },
        st: StartTable {
            table: vec![0u32; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 2,
            pattern_len: Some(1),
            universal_start_unanchored: Some(StateID(1)),
            universal_start_anchored: Some(StateID(1)),
        },
        special: Special {
            max: StateID(1),
            quit_id: StateID(0),
            min_match: StateID(1),
            max_match: StateID(1),
            min_accel: StateID(1),
            max_accel: StateID(1),
            min_start: StateID(0),
            max_start: StateID(1),
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: true,
            is_utf8: false,
            is_always_start_anchored: true,
        },
    };
    let mut output = Vec::new();
    dfa.fmt(&mut output);
}

#[test]
fn test_fmt_state_indicator_error() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![0u8; 10],
            classes: ByteClasses::default(),
            state_len: 2,
            pattern_len: 2,
        },
        st: StartTable {
            table: vec![1u32; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 2,
            pattern_len: Some(2),
            universal_start_unanchored: Some(StateID(1)),
            universal_start_anchored: Some(StateID(0)),
        },
        special: Special {
            max: StateID(2),
            quit_id: StateID(0),
            min_match: StateID(0),
            max_match: StateID(1),
            min_accel: StateID(0),
            max_accel: StateID(1),
            min_start: StateID(0),
            max_start: StateID(1),
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let mut output = Vec::new();
    dfa.fmt(&mut output);
}

