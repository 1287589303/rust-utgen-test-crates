// Answer 0

#[test]
fn test_start_state_with_none_look_behind_no() {
    let config = start::Config::new().anchored(Anchored::No);
    let dfa = DFA {
        tt: TransitionTable { table: vec![], classes: ByteClasses::default(), stride2: 1 },
        st: StartTable { table: vec![StateID(0); 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(0), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    let result = dfa.start_state(&config);
}

#[test]
fn test_start_state_with_none_look_behind_yes() {
    let config = start::Config::new().anchored(Anchored::Yes);
    let dfa = DFA {
        tt: TransitionTable { table: vec![], classes: ByteClasses::default(), stride2: 1 },
        st: StartTable { table: vec![StateID(0); 8], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special: Special { max: StateID(0), quit_id: StateID(0), min_match: StateID(0), max_match: StateID(0), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet::empty(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    let result = dfa.start_state(&config);
}

