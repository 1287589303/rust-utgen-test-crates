// Answer 0

#[test]
fn test_next_state_unchecked_valid_case() {
    let current = StateID(0);
    let input: u8 = 0;
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(0), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    unsafe { dfa.next_state_unchecked(current, input) };
}

#[test]
fn test_next_state_unchecked_max_state_id() {
    let current = StateID(0);
    let input: u8 = 255;
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(0), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    unsafe { dfa.next_state_unchecked(StateID(0), input) };
}

#[test]
fn test_next_state_unchecked_mid_range() {
    let current = StateID(0);
    let input: u8 = 128;
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(0), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    unsafe { dfa.next_state_unchecked(current, input) };
}

