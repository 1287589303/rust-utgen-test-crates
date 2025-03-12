// Answer 0

#[test]
fn test_write_to_correct_length() {
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    
    let transition_table = TransitionTable {
        table: vec![0; 64],
        classes: ByteClasses::default(),
        stride2: 3,
    };
    
    let start_table = StartTable {
        table: vec![1, 2, 3, 4],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    
    let match_states = MatchStates {
        slices: vec![0, 1],
        pattern_ids: vec![0, 1],
        pattern_len: 2,
    };
    
    let special = Special {
        max: 10,
        quit_id: 11,
        min_match: 12,
        max_match: 13,
        min_accel: 14,
        max_accel: 15,
        min_start: 16,
        max_start: 17,
    };
    
    let accels = Accels {
        accels: vec![0; 16],
    };
    
    let quitset = ByteSet::empty();
    
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

    let mut dst = vec![0u8; dfa.write_to_len()];
    let _ = dfa.write_to::<EndianImpl>(&mut dst);
}

#[test]
#[should_panic]
fn test_write_to_flags_err() {
    let flags = Flags {
        has_empty: true,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    
    let transition_table = TransitionTable {
        table: vec![0; 64],
        classes: ByteClasses::default(),
        stride2: 3,
    };
    
    let start_table = StartTable {
        table: vec![1, 2, 3, 4],
        kind: StartKind::Both,
        start_map: StartByteMap::default(),
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    
    let match_states = MatchStates {
        slices: vec![0, 1],
        pattern_ids: vec![0, 1],
        pattern_len: 2,
    };
    
    let special = Special {
        max: 10,
        quit_id: 11,
        min_match: 12,
        max_match: 13,
        min_accel: 14,
        max_accel: 15,
        min_start: 16,
        max_start: 17,
    };
    
    let accels = Accels {
        accels: vec![0; 16],
    };
    
    let quitset = ByteSet::empty();

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

    let mut dst = vec![0u8; dfa.write_to_len()];
    let _ = dfa.write_to::<EndianImpl>(&mut dst);
}

