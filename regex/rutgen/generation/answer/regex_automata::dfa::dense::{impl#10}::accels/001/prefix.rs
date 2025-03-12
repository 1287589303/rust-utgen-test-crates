// Answer 0

#[test]
fn test_accels_non_empty() {
    let accels_slice: &[u32] = &[1, 2, 3, 4, 5];
    let transition_table = TransitionTable {
        table: vec![0; 256],
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let start_table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: vec![0; 2],
        pattern_ids: vec![0; 1],
        pattern_len: 1,
    };
    let special = Special {
        max: 10,
        quit_id: 0,
        min_match: 1,
        max_match: 2,
        min_accel: 2,
        max_accel: 3,
        min_start: 0,
        max_start: 9,
    };
    let accels = Accels { accels: accels_slice };
    let quitset = ByteSet([false; 256]);
    let flags = Flags {
        has_empty: false,
        is_utf8: true,
        is_always_start_anchored: false,
    };
    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special,
        accels,
        pre: None,
        quitset,
        flags,
    };

    let result = dfa.accels();
}

#[test]
fn test_accels_empty() {
    let accels_slice: &[u32] = &[];
    let transition_table = TransitionTable {
        table: vec![0; 256],
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let start_table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let match_states = MatchStates {
        slices: vec![0; 0],
        pattern_ids: vec![0; 0],
        pattern_len: 0,
    };
    let special = Special {
        max: 0,
        quit_id: 0,
        min_match: 0,
        max_match: 0,
        min_accel: 0,
        max_accel: 0,
        min_start: 0,
        max_start: 0,
    };
    let accels = Accels { accels: accels_slice };
    let quitset = ByteSet([false; 256]);
    let flags = Flags {
        has_empty: true,
        is_utf8: true,
        is_always_start_anchored: false,
    };
    let dfa = DFA {
        tt: transition_table,
        st: start_table,
        ms: match_states,
        special,
        accels,
        pre: None,
        quitset,
        flags,
    };

    let result = dfa.accels();
}

