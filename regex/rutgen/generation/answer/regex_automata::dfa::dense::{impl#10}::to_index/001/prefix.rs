// Answer 0

#[test]
fn test_to_index_valid_non_premultiplied() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4],
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let dfa = DFA {
        tt: transition_table,
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            pattern_len: 0,
        },
        special: Special::default(),
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };

    let state_id = StateID(0);
    dfa.to_index(state_id);
}

#[test]
fn test_to_index_valid_non_premultiplied_max() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4],
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let dfa = DFA {
        tt: transition_table,
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            pattern_len: 0,
        },
        special: Special::default(),
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };

    let state_id = StateID(4);
    dfa.to_index(state_id);
}

#[test]
fn test_to_index_invalid_large() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4],
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let dfa = DFA {
        tt: transition_table,
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            pattern_len: 0,
        },
        special: Special::default(),
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };

    let state_id = StateID(5);
    dfa.to_index(state_id);
}

#[test]
fn test_to_index_valid_premultiplied() {
    let transition_table = TransitionTable {
        table: vec![0, 1, 2, 3, 4],
        classes: ByteClasses::default(),
        stride2: 1,
    };
    let dfa = DFA {
        tt: transition_table,
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: StartByteMap::default(),
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        },
        ms: MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            pattern_len: 0,
        },
        special: Special::default(),
        accels: Accels { accels: vec![] },
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags::default(),
    };

    let state_id = StateID(2);
    dfa.to_index(state_id);
}

