// Answer 0

#[test]
fn test_fmt_empty_states() {
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    let special = Special {
        max: 0,
        quit_id: 0,
        min_match: 0,
        max_match: 0,
        min_accel: 0,
        max_accel: 0,
        min_start: 0,
        max_start: 0,
    };

    let start_table = StartTable {
        table: vec![0u32; 8],
        kind: StartKind::NonWordByte,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: Some(StateID(1)),
        universal_start_anchored: None,
    };

    let transitions = Transitions {
        sparse: vec![],
        classes: ByteClasses::default(),
        state_len: 0,
        pattern_len: 0,
    };

    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset: ByteSet([false; 256]),
        flags,
    };

    let _ = format!("{:?}", dfa);
}

#[test]
fn test_fmt_non_empty_start_states() {
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    let special = Special {
        max: 0,
        quit_id: 0,
        min_match: 0,
        max_match: 0,
        min_accel: 0,
        max_accel: 0,
        min_start: 0,
        max_start: 0,
    };

    let start_table = StartTable {
        table: vec![1u32; 8],
        kind: StartKind::WordByte,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(0),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let transitions = Transitions {
        sparse: vec![],
        classes: ByteClasses::default(),
        state_len: 0,
        pattern_len: 0,
    };

    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special,
        pre: None,
        quitset: ByteSet([false; 256]),
        flags,
    };

    let _ = format!("{:?}", dfa);
}

