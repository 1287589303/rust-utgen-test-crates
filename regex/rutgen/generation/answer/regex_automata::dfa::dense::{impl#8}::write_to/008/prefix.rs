// Answer 0

#[test]
fn test_write_to_little_endian() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::default(), stride2: 2},
        st: StartTable { table: vec![0; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 1, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![0; 4], pattern_ids: vec![0; 4], pattern_len: 1 },
        special: Special { max: 10, quit_id: 0, min_match: 0, max_match: 1, min_accel: 0, max_accel: 1, min_start: 0, max_start: 1 },
        accels: Accels { accels: vec![0; 4] },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: true, is_utf8: true, is_always_start_anchored: false },
    };

    let mut dst = vec![0; dfa.write_to_len()];
    let result = dfa.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_big_endian() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::default(), stride2: 2 },
        st: StartTable { table: vec![0; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 1, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![0; 4], pattern_ids: vec![0; 4], pattern_len: 1 },
        special: Special { max: 10, quit_id: 0, min_match: 0, max_match: 1, min_accel: 0, max_accel: 1, min_start: 0, max_start: 1 },
        accels: Accels { accels: vec![0; 4] },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: true, is_utf8: true, is_always_start_anchored: false },
    };

    let mut dst = vec![0; dfa.write_to_len()];
    let result = dfa.write_to::<BigEndian>(&mut dst);
}

#[test]
fn test_write_to_native_endian() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![0; 10], classes: ByteClasses::default(), stride2: 2 },
        st: StartTable { table: vec![0; 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 1, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![0; 4], pattern_ids: vec![0; 4], pattern_len: 1 },
        special: Special { max: 10, quit_id: 0, min_match: 0, max_match: 1, min_accel: 0, max_accel: 1, min_start: 0, max_start: 1 },
        accels: Accels { accels: vec![0; 4] },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: true, is_utf8: true, is_always_start_anchored: false },
    };

    let mut dst = vec![0; dfa.write_to_len()];
    let result = dfa.write_to::<NativeEndian>(&mut dst);
}

