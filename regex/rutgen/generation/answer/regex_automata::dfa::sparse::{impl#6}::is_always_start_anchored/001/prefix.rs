// Answer 0

#[test]
fn test_is_always_start_anchored_true() {
    let flags = Flags {
        has_empty: false,
        is_utf8: true,
        is_always_start_anchored: true,
    };
    
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 4, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };

    dfa.is_always_start_anchored();
}

#[test]
fn test_is_always_start_anchored_false() {
    let flags = Flags {
        has_empty: false,
        is_utf8: true,
        is_always_start_anchored: false,
    };
    
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 4, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: 0, quit_id: 0, min_match: 0, max_match: 0, min_accel: 0, max_accel: 0, min_start: 0, max_start: 0 },
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };

    dfa.is_always_start_anchored();
}

