// Answer 0

#[test]
fn test_to_bytes_little_endian_vec() {
    let dfa = DFA {
        tt: Transitions { sparse: vec![0u8; 10], classes: ByteClasses::default(), state_len: 1, pattern_len: 1 },
        st: StartTable { table: vec![0u32; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 2, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 1, quit_id: 0, min_match: 1, max_match: 1, min_accel: 1, max_accel: 1, min_start: 1, max_start: 1 },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let _ = dfa.to_bytes::<wire::LittleEndian>();
}

#[test]
fn test_to_bytes_big_endian_vec() {
    let dfa = DFA {
        tt: Transitions { sparse: vec![0u8; 10], classes: ByteClasses::default(), state_len: 1, pattern_len: 1 },
        st: StartTable { table: vec![0u32; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 2, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 1, quit_id: 0, min_match: 1, max_match: 1, min_accel: 1, max_accel: 1, min_start: 1, max_start: 1 },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let _ = dfa.to_bytes::<wire::BigEndian>();
}

#[test]
fn test_to_bytes_native_endian_vec() {
    let dfa = DFA {
        tt: Transitions { sparse: vec![0u8; 10], classes: ByteClasses::default(), state_len: 1, pattern_len: 1 },
        st: StartTable { table: vec![0u32; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 2, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 1, quit_id: 0, min_match: 1, max_match: 1, min_accel: 1, max_accel: 1, min_start: 1, max_start: 1 },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let _ = dfa.to_bytes::<wire::NativeEndian>();
}

