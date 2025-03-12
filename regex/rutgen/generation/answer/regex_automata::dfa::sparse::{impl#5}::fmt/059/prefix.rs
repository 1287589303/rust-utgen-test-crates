// Answer 0

#[test]
fn test_fmt_empty_states() {
    let formatter = &mut fmt::Formatter::new();
    let empty_transitions: Transitions<Vec<u8>> = Transitions {
        sparse: vec![],
        classes: ByteClasses::default(),
        state_len: 1,
        pattern_len: 0,
    };
    let empty_start_table: StartTable<Vec<u32>> = StartTable {
        table: vec![0; 8],
        kind: StartKind::NonWordByte,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let flags = Flags { has_empty: true, is_utf8: true, is_always_start_anchored: false };
    let dfa = DFA {
        tt: empty_transitions,
        st: empty_start_table,
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags,
    };

    let _ = dfa.fmt(formatter);
}

#[test]
#[should_panic]
fn test_fmt_no_start_states() {
    let formatter = &mut fmt::Formatter::new();
    let transitions = Transitions {
        sparse: vec![0; 1],
        classes: ByteClasses::default(),
        state_len: 1,
        pattern_len: 0,
    };
    let start_table: StartTable<Vec<u32>> = StartTable {
        table: vec![],
        kind: StartKind::NonWordByte,
        start_map: StartByteMap::default(),
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let flags = Flags { has_empty: true, is_utf8: true, is_always_start_anchored: false };
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags,
    };

    let _ = dfa.fmt(formatter);
}

#[test]
fn test_fmt_invalid_start_id() {
    let formatter = &mut fmt::Formatter::new();
    let transitions = Transitions {
        sparse: vec![0; 4],
        classes: ByteClasses::default(),
        state_len: 1,
        pattern_len: 0,
    };
    let start_table: StartTable<Vec<u32>> = StartTable {
        table: vec![1u32; 8],
        kind: StartKind::NonWordByte,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let flags = Flags { has_empty: true, is_utf8: false, is_always_start_anchored: false };
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags,
    };

    let _ = dfa.fmt(formatter);
}

