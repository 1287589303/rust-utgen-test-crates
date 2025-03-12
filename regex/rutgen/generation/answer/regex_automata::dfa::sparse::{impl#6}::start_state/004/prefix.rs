// Answer 0

#[test]
fn test_start_state_no_look_behind_unanchored() {
    let input = Start::Text;
    let config = Config::new().anchored(Anchored::No);
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::new(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![0], kind: StartKind::Both, start_map: StartByteMap::new(&LookMatcher::default()), stride: 1, pattern_len: Some(0) },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(1), min_accel: StateID(0), max_accel: StateID(1), min_start: StateID(0), max_start: StateID(1) },
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let _ = dfa.start_state(&config);
}

#[test]
fn test_start_state_no_look_behind_anchored() {
    let input = Start::Text;
    let config = Config::new().anchored(Anchored::Yes);
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::new(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![0], kind: StartKind::Both, start_map: StartByteMap::new(&LookMatcher::default()), stride: 1, pattern_len: Some(0) },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(1), min_accel: StateID(0), max_accel: StateID(1), min_start: StateID(0), max_start: StateID(1) },
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let _ = dfa.start_state(&config);
}

#[test]
fn test_start_state_no_look_behind_pattern() {
    let input = Start::Text;
    let config = Config::new().anchored(Anchored::Pattern(PatternID(0)));
    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::new(), state_len: 1, pattern_len: 0 },
        st: StartTable { table: vec![0], kind: StartKind::Both, start_map: StartByteMap::new(&LookMatcher::default()), stride: 1, pattern_len: Some(1) },
        special: Special { max: StateID(1), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(1), min_accel: StateID(0), max_accel: StateID(1), min_start: StateID(0), max_start: StateID(1) },
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };
    let _ = dfa.start_state(&config);
}

