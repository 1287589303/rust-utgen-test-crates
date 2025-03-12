// Answer 0

#[test]
fn test_has_empty_true() {
    let dfa = DFA {
        tt: TransitionTable {
            table: vec![],
            classes: ByteClasses::new(),
            stride2: 1,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::new(),
            stride: 1,
            pattern_len: Some(0),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            pattern_len: 0,
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
        accels: Accels {
            accels: vec![],
        },
        pre: None,
        quitset: ByteSet([false; 256]),
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
        tt: TransitionTable {
            table: vec![],
            classes: ByteClasses::new(),
            stride2: 1,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::new(),
            stride: 1,
            pattern_len: Some(0),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            pattern_len: 0,
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
        accels: Accels {
            accels: vec![],
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };

    let _ = dfa.has_empty();
}

