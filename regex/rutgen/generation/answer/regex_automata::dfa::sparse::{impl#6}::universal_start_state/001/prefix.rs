// Answer 0

#[test]
fn test_universal_start_state_pattern() {
    struct DummyDFA {
        st: StartTable<Vec<u32>>,
    }

    let dfa = DummyDFA {
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: vec![],
            stride: 0,
            pattern_len: None,
            universal_start_unanchored: Some(StateID(1)),
            universal_start_anchored: Some(StateID(2)),
        },
    };

    let result = dfa.universal_start_state(Anchored::Pattern(PatternID(0)));
}

#[test]
fn test_universal_start_state_pattern_non_empty() {
    struct DummyDFA {
        st: StartTable<Vec<u32>>,
    }

    let dfa = DummyDFA {
        st: StartTable {
            table: vec![0; 8],
            kind: StartKind::Both,
            start_map: vec![],
            stride: 1,
            pattern_len: Some(1),
            universal_start_unanchored: Some(StateID(1)),
            universal_start_anchored: Some(StateID(2)),
        },
    };

    let result = dfa.universal_start_state(Anchored::Pattern(PatternID(1)));
}

#[test]
fn test_universal_start_state_pattern_with_large_id() {
    struct DummyDFA {
        st: StartTable<Vec<u32>>,
    }

    let dfa = DummyDFA {
        st: StartTable {
            table: vec![],
            kind: StartKind::Both,
            start_map: vec![],
            stride: 0,
            pattern_len: Some(5),
            universal_start_unanchored: Some(StateID(1)),
            universal_start_anchored: Some(StateID(2)),
        },
    };

    let result = dfa.universal_start_state(Anchored::Pattern(PatternID(4)));
}

