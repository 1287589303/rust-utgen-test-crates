// Answer 0

#[test]
fn test_start_kind_both() {
    let start_table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 2,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::default(), stride2: 3 },
        st: start_table,
        ms: MatchStates { slices: vec![0; 10], pattern_ids: vec![0; 10], pattern_len: 2 },
        special: Special { max: 2, quit_id: 1, min_match: 0, max_match: 2, min_accel: 0, max_accel: 1, min_start: 0, max_start: 1 },
        accels: Accels { accels: vec![0; 5] },
        pre: None,
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };

    let _ = dfa.start_kind();
}

#[test]
fn test_start_kind_unanchored() {
    let start_table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Unanchored,
        start_map: StartByteMap::default(),
        stride: 2,
        pattern_len: None,
        universal_start_unanchored: Some(0),
        universal_start_anchored: None,
    };

    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::default(), stride2: 2 },
        st: start_table,
        ms: MatchStates { slices: vec![0; 5], pattern_ids: vec![1; 5], pattern_len: 5 },
        special: Special { max: 3, quit_id: 2, min_match: 1, max_match: 3, min_accel: 1, max_accel: 2, min_start: 0, max_start: 3 },
        accels: Accels { accels: vec![1; 5] },
        pre: Some(Prefilter { pre: Arc::new(MockPrefilter), is_fast: true, max_needle_len: 5 }),
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: true, is_utf8: false, is_always_start_anchored: false },
    };

    let _ = dfa.start_kind();
}

#[test]
fn test_start_kind_anchored() {
    let start_table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Anchored,
        start_map: StartByteMap::default(),
        stride: 2,
        pattern_len: Some(0),
        universal_start_unanchored: None,
        universal_start_anchored: Some(1),
    };

    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::default(), stride2: 4 },
        st: start_table,
        ms: MatchStates { slices: vec![1; 5], pattern_ids: vec![0; 5], pattern_len: 3 },
        special: Special { max: 1, quit_id: 1, min_match: 0, max_match: 1, min_accel: 0, max_accel: 1, min_start: 0, max_start: 0 },
        accels: Accels { accels: vec![0; 2] },
        pre: None,
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: true },
    };

    let _ = dfa.start_kind();
}

