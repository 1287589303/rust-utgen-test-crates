// Answer 0

#[test]
fn test_dfa_to_owned_with_vec_u8() {
    let tt = Transitions {
        sparse: vec![1, 2, 3],
        classes: ByteClasses::default(),
        state_len: 5,
        pattern_len: 2,
    };
    let st = StartTable {
        table: vec![0, 1, 2, 3, 4, 5, 6, 7],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: Some(1),
        universal_start_anchored: Some(2),
    };
    let flags = Flags {
        has_empty: true,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    let special = Special {
        max: 4,
        quit_id: 5,
        min_match: 1,
        max_match: 3,
        min_accel: 0,
        max_accel: 2,
        min_start: 0,
        max_start: 4,
    };
    let dfa: DFA<&[u8]> = DFA {
        tt,
        st,
        special,
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };
    let owned_dfa = dfa.to_owned();
}

#[test]
fn test_dfa_to_owned_with_slice() {
    let tt = Transitions {
        sparse: &[1, 2, 3],
        classes: ByteClasses::default(),
        state_len: 5,
        pattern_len: 2,
    };
    let st = StartTable {
        table: &[0, 1, 2, 3, 4, 5, 6, 7],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: Some(1),
        universal_start_anchored: Some(2),
    };
    let flags = Flags {
        has_empty: true,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    let special = Special {
        max: 4,
        quit_id: 5,
        min_match: 1,
        max_match: 3,
        min_accel: 0,
        max_accel: 2,
        min_start: 0,
        max_start: 4,
    };
    let dfa: DFA<&[u8]> = DFA {
        tt,
        st,
        special,
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };
    let owned_dfa = dfa.to_owned();
}

