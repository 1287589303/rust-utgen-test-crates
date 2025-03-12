// Answer 0

#[test]
fn test_has_empty_true() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 1,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 1,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: 0,
            quit_id: 0,
            min_match: 0,
            max_match: 0,
            min_accel: 0,
            max_accel: 0,
            min_start: 0,
            max_start: 0,
        },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: true,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };
    let _ = dfa.has_empty();
}

#[test]
fn test_has_empty_false() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 1,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 1,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: 0,
            quit_id: 0,
            min_match: 0,
            max_match: 0,
            min_accel: 0,
            max_accel: 0,
            min_start: 0,
            max_start: 0,
        },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };
    let _ = dfa.has_empty();
}

