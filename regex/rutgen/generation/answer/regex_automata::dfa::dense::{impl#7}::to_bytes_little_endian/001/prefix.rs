// Answer 0

#[test]
fn test_to_bytes_little_endian_valid_dfa() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4],
        classes: ByteClasses::default(),
        stride2: 2,
    };
    let start_table = StartTable {
        table: vec![0, 1, 2, 3, 4, 5, 6, 7],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: vec![0, 1, 2],
        pattern_ids: vec![0],
        pattern_len: 1,
    };
    let special_states = Special {
        max: 2,
        quit_id: 1,
        min_match: 1,
        max_match: 3,
        min_accel: 0,
        max_accel: 2,
        min_start: 0,
        max_start: 4,
    };
    let accels = Accels {
        accels: vec![0, 1],
    };
    let flags = Flags {
        has_empty: false,
        is_utf8: true,
        is_always_start_anchored: false,
    };
    
    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special: special_states,
        accels: accels,
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };
    
    let (bytes, padding) = dfa.to_bytes_little_endian();
}

#[test]
fn test_to_bytes_little_endian_empty_dfa() {
    let transition_table = TransitionTable {
        table: Vec::new(),
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let start_table = StartTable {
        table: Vec::new(),
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 0,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: Vec::new(),
        pattern_ids: Vec::new(),
        pattern_len: 0,
    };
    let special_states = Special {
        max: 0,
        quit_id: 0,
        min_match: 0,
        max_match: 0,
        min_accel: 0,
        max_accel: 0,
        min_start: 0,
        max_start: 0,
    };
    let accels = Accels {
        accels: Vec::new(),
    };
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: true,
    };

    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special: special_states,
        accels: accels,
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };

    let (bytes, padding) = dfa.to_bytes_little_endian();
}

#[test]
fn test_to_bytes_little_endian_with_accelerators() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4, 5],
        classes: ByteClasses::default(),
        stride2: 3,
    };
    let start_table = StartTable {
        table: vec![0, 1, 2, 3, 4, 5, 6, 7],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 5,
        pattern_len: Some(2),
        universal_start_unanchored: Some(0),
        universal_start_anchored: Some(1),
    };
    let match_states = MatchStates {
        slices: vec![0, 1, 2],
        pattern_ids: vec![0, 1],
        pattern_len: 2,
    };
    let special_states = Special {
        max: 3,
        quit_id: 2,
        min_match: 1,
        max_match: 4,
        min_accel: 0,
        max_accel: 1,
        min_start: 0,
        max_start: 5,
    };
    let accels = Accels {
        accels: vec![1, 2, 3],
    };
    let flags = Flags {
        has_empty: true,
        is_utf8: true,
        is_always_start_anchored: false,
    };

    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special: special_states,
        accels: accels,
        pre: None,
        quitset: ByteSet::default(),
        flags,
    };

    let (bytes, padding) = dfa.to_bytes_little_endian();
}

