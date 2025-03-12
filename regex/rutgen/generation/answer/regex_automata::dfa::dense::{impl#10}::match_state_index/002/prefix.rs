// Answer 0

#[test]
fn test_match_state_index_non_match_state_below_min() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![], classes: ByteClasses::default(), stride2: 1 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special: Special { max: StateID(10), quit_id: StateID(0), min_match: StateID(5), max_match: StateID(10), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    let non_match_id = StateID(4); // below min_match
    let _index = dfa.match_state_index(non_match_id);
}

#[test]
fn test_match_state_index_non_match_state_at_min() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![], classes: ByteClasses::default(), stride2: 1 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special: Special { max: StateID(10), quit_id: StateID(0), min_match: StateID(5), max_match: StateID(10), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    let non_match_id = StateID(5); // at min_match
    let _index = dfa.match_state_index(non_match_id);
}

#[test]
fn test_match_state_index_non_match_state_above_max() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![], classes: ByteClasses::default(), stride2: 1 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special: Special { max: StateID(10), quit_id: StateID(0), min_match: StateID(5), max_match: StateID(10), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    let non_match_id = StateID(11); // above max_match
    let _index = dfa.match_state_index(non_match_id);
}

#[test]
fn test_match_state_index_non_match_state_beyond_max() {
    let dfa = DFA {
        tt: TransitionTable { table: vec![], classes: ByteClasses::default(), stride2: 1 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 0, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        special: Special { max: StateID(10), quit_id: StateID(0), min_match: StateID(5), max_match: StateID(10), min_accel: StateID(0), max_accel: StateID(0), min_start: StateID(0), max_start: StateID(0) },
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet { bits: BitSet::default() },
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };
    let non_match_id = StateID(100); // clearly beyond the range
    let _index = dfa.match_state_index(non_match_id);
}

