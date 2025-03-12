// Answer 0

#[test]
fn test_is_utf8_true() {
    let dfa = DFA {
        tt: Transitions { sparse: vec![] },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::new(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 0, quit_id: 1, min_match: 2, max_match: 3, min_accel: 4, max_accel: 5, min_start: 6, max_start: 7 },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };

    let _ = dfa.is_utf8();
}

#[test]
fn test_is_utf8_false() {
    let dfa = DFA {
        tt: Transitions { sparse: vec![] },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::new(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 0, quit_id: 1, min_match: 2, max_match: 3, min_accel: 4, max_accel: 5, min_start: 6, max_start: 7 },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };

    let _ = dfa.is_utf8();
}

