// Answer 0

#[test]
fn test_write_to_with_exact_length_buffer() {
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
            stride: 8,
            pattern_len: Some(1),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![0; 1],
            pattern_ids: vec![0; 1],
            pattern_len: 1,
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
            accels: vec![0; 1],
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
    let mut dst = vec![0u8; nwrite];
    
    let _result = dfa.write_to::<u32>(&mut dst);
}

#[test]
#[should_panic(expected = "buffer too small")]
fn test_write_to_with_too_small_buffer() {
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
            stride: 8,
            pattern_len: Some(1),
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![0; 1],
            pattern_ids: vec![0; 1],
            pattern_len: 1,
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
            accels: vec![0; 1],
        },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags {
            has_empty: false,
            is_utf8: true,
            is_always_start_anchored: false,
        },
    };

    // Buffer size is intentionally lower than required
    let smaller_buf_size = dfa.write_to_len() - 1;
    let mut dst = vec![0u8; smaller_buf_size];

    let _result = dfa.write_to::<u32>(&mut dst);
}

