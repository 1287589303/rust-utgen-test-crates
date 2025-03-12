// Answer 0

#[test]
fn test_memory_usage_empty() {
    let transitions = Transitions {
        sparse: vec![],
        classes: ByteClasses::default(),
        state_len: 0,
        pattern_len: 0,
    };
    let start_table = StartTable {
        table: vec![],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let dfa = DFA {
        tt: transitions,
        st: start_table,
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
    let _ = dfa.memory_usage();
}

#[test]
fn test_memory_usage_small() {
    let transitions = Transitions {
        sparse: vec![1, 2, 3, 4],
        classes: ByteClasses::default(),
        state_len: 4,
        pattern_len: 1,
    };
    let start_table = StartTable {
        table: vec![0, 1, 2, 3],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: Some(1),
        universal_start_anchored: Some(0),
    };
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special: Special {
            max: 4,
            quit_id: 2,
            min_match: 1,
            max_match: 3,
            min_accel: 1,
            max_accel: 3,
            min_start: 0,
            max_start: 3,
        },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: true,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let _ = dfa.memory_usage();
}

#[test]
fn test_memory_usage_large() {
    let transitions = Transitions {
        sparse: vec![0; (u32::MAX as usize)],
        classes: ByteClasses::default(),
        state_len: u32::MAX as usize,
        pattern_len: u32::MAX as usize,
    };
    let start_table = StartTable {
        table: vec![0; (u32::MAX as usize)],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: u32::MAX as usize,
        pattern_len: Some(u32::MAX as usize),
        universal_start_unanchored: Some(u32::MAX),
        universal_start_anchored: Some(u32::MAX),
    };
    let dfa = DFA {
        tt: transitions,
        st: start_table,
        special: Special {
            max: u32::MAX,
            quit_id: u32::MAX,
            min_match: u32::MAX,
            max_match: u32::MAX,
            min_accel: u32::MAX,
            max_accel: u32::MAX,
            min_start: u32::MAX,
            max_start: u32::MAX,
        },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags {
            has_empty: true,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let _ = dfa.memory_usage();
}

