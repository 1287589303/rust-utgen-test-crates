// Answer 0

#[test]
fn test_to_sparse_valid() {
    let dfa = DFA {
        tt: TransitionTable {
            table: vec![0; 256],
            classes: ByteClasses::default(),
            stride2: 8,
        },
        st: StartTable {
            table: vec![0; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 4,
            pattern_len: Some(1),
            universal_start_unanchored: Some(0),
            universal_start_anchored: Some(1),
        },
        ms: MatchStates {
            slices: vec![(0, 1)],
            pattern_ids: vec![0],
            pattern_len: 1,
        },
        special: Special {
            max: 10,
            quit_id: 9,
            min_match: 8,
            max_match: 9,
            min_accel: 6,
            max_accel: 7,
            min_start: 0,
            max_start: 10,
        },
        accels: Accels {
            accels: vec![0u8; 4],
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let _ = dfa.to_sparse();
}

#[test]
#[should_panic]
fn test_to_sparse_invalid_state() {
    let dfa = DFA {
        tt: TransitionTable {
            table: vec![0; 256],
            classes: ByteClasses::default(),
            stride2: 8,
        },
        st: StartTable {
            table: vec![0; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 4,
            pattern_len: Some(1),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![(0, 1)],
            pattern_ids: vec![0],
            pattern_len: 1,
        },
        special: Special {
            max: 10,
            quit_id: 9,
            min_match: 8,
            max_match: 9,
            min_accel: 6,
            max_accel: 7,
            min_start: 0,
            max_start: 10,
        },
        accels: Accels {
            accels: vec![0u8; 4],
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: true,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let _ = dfa.to_sparse();
}

#[test]
fn test_to_sparse_empty_dfa() {
    let dfa = DFA {
        tt: TransitionTable {
            table: vec![],
            classes: ByteClasses::default(),
            stride2: 1,
        },
        st: StartTable {
            table: vec![0; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
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
    let _ = dfa.to_sparse();
}

