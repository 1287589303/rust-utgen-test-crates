// Answer 0

#[test]
fn test_write_to_little_endian() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![0u8; 4],
            classes: ByteClasses::new(),
            state_len: 1,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![0u32; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 4,
            pattern_len: Some(0),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special::new(),
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags {
            has_empty: true,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let mut buffer = vec![0u8; 128];
    let _ = dfa.write_to::<u32>(&mut buffer);
}

#[test]
fn test_write_to_big_endian() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![0u8; 4],
            classes: ByteClasses::new(),
            state_len: 1,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![0u32; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 4,
            pattern_len: Some(0),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special::new(),
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags {
            has_empty: true,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let mut buffer = vec![0u8; 128];
    let _ = dfa.write_to::<u32>(&mut buffer);
}

#[test]
fn test_write_to_native_endian() {
    let dfa = DFA {
        tt: Transitions {
            sparse: vec![0u8; 4],
            classes: ByteClasses::new(),
            state_len: 1,
            pattern_len: 0,
        },
        st: StartTable {
            table: vec![0u32; 8],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 4,
            pattern_len: Some(0),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        special: Special::new(),
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags {
            has_empty: true,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    let mut buffer = vec![0u8; 128];
    let _ = dfa.write_to::<u32>(&mut buffer);
}

