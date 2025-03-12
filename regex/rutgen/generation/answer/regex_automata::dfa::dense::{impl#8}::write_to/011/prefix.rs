// Answer 0

#[test]
fn test_write_to_little_endian() {
    let dfa = DFA {
        tt: TransitionTable {
            table: vec![0; 10],
            classes: ByteClasses::default(),
            stride2: 4,
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
            slices: vec![0; 10],
            pattern_ids: vec![0; 10],
            pattern_len: 1,
        },
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 3,
            min_accel: 4,
            max_accel: 5,
            min_start: 6,
            max_start: 7,
        },
        accels: Accels {
            accels: vec![0; 10],
        },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags {
            has_empty: false,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    
    let nwrite = dfa.write_to_len();
    let mut dst = vec![0; nwrite];
    
    let result = dfa.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_big_endian() {
    let dfa = DFA {
        tt: TransitionTable {
            table: vec![0; 10],
            classes: ByteClasses::default(),
            stride2: 4,
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
            slices: vec![0; 10],
            pattern_ids: vec![0; 10],
            pattern_len: 1,
        },
        special: Special {
            max: 10,
            quit_id: 1,
            min_match: 2,
            max_match: 3,
            min_accel: 4,
            max_accel: 5,
            min_start: 6,
            max_start: 7,
        },
        accels: Accels {
            accels: vec![0; 10],
        },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags {
            has_empty: false,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };
    
    let nwrite = dfa.write_to_len();
    let mut dst = vec![0; nwrite];
    
    let result = dfa.write_to::<BigEndian>(&mut dst);
}

