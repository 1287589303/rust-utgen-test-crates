// Answer 0

#[test]
fn test_to_bytes_little_endian() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::new(), stride2: 4 },
        st: StartTable { table: vec![0; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 2, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![0; 2], pattern_ids: vec![1, 2], pattern_len: 2 },
        special: Special { max: 10, quit_id: 0, min_match: 1, max_match: 2, min_accel: 3, max_accel: 4, min_start: 0, max_start: 2 },
        accels: Accels { accels: vec![0; 5] },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let (buf, padding) = dfa.to_bytes::<LittleEndian>();
}

#[test]
fn test_to_bytes_big_endian() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 15], classes: ByteClasses::new(), stride2: 3 },
        st: StartTable { table: vec![0; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 1, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![0; 2], pattern_ids: vec![1, 2], pattern_len: 2 },
        special: Special { max: 15, quit_id: 0, min_match: 1, max_match: 2, min_accel: 3, max_accel: 4, min_start: 0, max_start: 2 },
        accels: Accels { accels: vec![0; 5] },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags { has_empty: true, is_utf8: true, is_always_start_anchored: false },
    };
    let (buf, padding) = dfa.to_bytes::<BigEndian>();
}

#[test]
fn test_to_bytes_native_endian() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 20], classes: ByteClasses::new(), stride2: 5 },
        st: StartTable { table: vec![0; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 3, pattern_len: Some(1), universal_start_unanchored: Some(1), universal_start_anchored: Some(2) },
        ms: MatchStates { slices: vec![0; 2], pattern_ids: vec![1, 2], pattern_len: 2 },
        special: Special { max: 20, quit_id: 0, min_match: 1, max_match: 3, min_accel: 4, max_accel: 5, min_start: 0, max_start: 2 },
        accels: Accels { accels: vec![0; 5] },
        pre: None,
        quitset: ByteSet([false; 256]),
        flags: Flags { has_empty: true, is_utf8: true, is_always_start_anchored: true },
    };
    let (buf, padding) = dfa.to_bytes::<NativeEndian>();
}

