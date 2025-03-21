// Answer 0

#[test]
fn test_is_dead_state_valid() {
    let state_id_0 = StateID(0);
    let state_id_1 = StateID(1);
    let state_id_2 = StateID(2);

    let special = Special {
        max: state_id_2,
        quit_id: state_id_0,
        min_match: state_id_1,
        max_match: state_id_2,
        min_accel: state_id_1,
        max_accel: state_id_2,
        min_start: state_id_1,
        max_start: state_id_2,
    };

    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 3, pattern_len: 3 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 1, pattern_len: Some(1), universal_start_unanchored: None, universal_start_anchored: None },
        special,
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };

    dfa.is_dead_state(state_id_0);
    dfa.is_dead_state(state_id_1);
    dfa.is_dead_state(state_id_2);
}

#[test]
fn test_is_dead_state_dead_case() {
    let state_id_dead = StateID(0);
    let state_id_non_dead = StateID(1);

    let special = Special {
        max: state_id_non_dead,
        quit_id: state_id_dead,
        min_match: state_id_non_dead,
        max_match: state_id_non_dead,
        min_accel: state_id_non_dead,
        max_accel: state_id_non_dead,
        min_start: state_id_non_dead,
        max_start: state_id_non_dead,
    };

    let dfa = DFA {
        tt: Transitions { sparse: vec![], classes: ByteClasses::default(), state_len: 2, pattern_len: 1 },
        st: StartTable { table: vec![], kind: StartKind::Both, start_map: StartByteMap::default(), stride: 1, pattern_len: None, universal_start_unanchored: None, universal_start_anchored: None },
        special,
        pre: None,
        quitset: ByteSet::default(),
        flags: Flags { has_empty: false, is_utf8: false, is_always_start_anchored: false },
    };

    dfa.is_dead_state(state_id_dead);
    dfa.is_dead_state(state_id_non_dead);
}

