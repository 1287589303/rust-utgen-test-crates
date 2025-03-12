// Answer 0

#[test]
fn test_fmt_empty_states() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 0,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: StateID(SmallIndex::default()),
            quit_id: StateID(SmallIndex::default()),
            min_match: StateID(SmallIndex::default()),
            max_match: StateID(SmallIndex::default()),
            min_accel: StateID(SmallIndex::default()),
            max_accel: StateID(SmallIndex::default()),
            min_start: StateID(SmallIndex::default()),
            max_start: StateID(SmallIndex::default()),
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };
    
    let _ = format!("{:?}", dfa);
}

#[test]
fn test_fmt_no_states() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 0,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: StateID(SmallIndex::default()),
            quit_id: StateID(SmallIndex::default()),
            min_match: StateID(SmallIndex::default()),
            max_match: StateID(SmallIndex::default()),
            min_accel: StateID(SmallIndex::default()),
            max_accel: StateID(SmallIndex::default()),
            min_start: StateID(SmallIndex::default()),
            max_start: StateID(SmallIndex::default()),
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };
    
    let _ = format!("{:?}", dfa);
}

#[test]
fn test_fmt_start_table_empty() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![],
            classes: ByteClasses::default(),
            state_len: 0,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special {
            max: StateID(SmallIndex::default()),
            quit_id: StateID(SmallIndex::default()),
            min_match: StateID(SmallIndex::default()),
            max_match: StateID(SmallIndex::default()),
            min_accel: StateID(SmallIndex::default()),
            max_accel: StateID(SmallIndex::default()),
            min_start: StateID(SmallIndex::default()),
            max_start: StateID(SmallIndex::default()),
        },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags {
            has_empty: false,
            is_utf8: false,
            is_always_start_anchored: false,
        },
    };
    
    let _ = format!("{:?}", dfa);
}

