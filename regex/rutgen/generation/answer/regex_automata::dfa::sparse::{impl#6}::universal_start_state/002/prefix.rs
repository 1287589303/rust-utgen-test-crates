// Answer 0

#[cfg(test)]
fn test_universal_start_state_anchored() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }
    
    impl TestDFA {
        fn new() -> Self {
            let start_table = StartTable {
                table: vec![0, 1, 2, 3, 4, 5, 6, 7],
                kind: StartKind::Both,
                start_map: StartByteMap::new(),
                stride: 8,
                pattern_len: Some(2),
                universal_start_unanchored: Some(StateID(1)),
                universal_start_anchored: Some(StateID(2)),
            };
            Self { st: start_table }
        }
    }

    let dfa = TestDFA::new();
    let mode = Anchored::Yes;
    let result = dfa.universal_start_state(mode);
}

#[cfg(test)]
fn test_universal_start_state_no() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }

    impl TestDFA {
        fn new() -> Self {
            let start_table = StartTable {
                table: vec![0, 1, 2, 3, 4, 5, 6, 7],
                kind: StartKind::Both,
                start_map: StartByteMap::new(),
                stride: 8,
                pattern_len: Some(2),
                universal_start_unanchored: Some(StateID(1)),
                universal_start_anchored: None,
            };
            Self { st: start_table }
        }
    }

    let dfa = TestDFA::new();
    let mode = Anchored::No;
    let result = dfa.universal_start_state(mode);
}

#[cfg(test)]
fn test_universal_start_state_pattern() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }

    impl TestDFA {
        fn new() -> Self {
            let start_table = StartTable {
                table: vec![0, 1, 2, 3, 4, 5, 6, 7],
                kind: StartKind::Both,
                start_map: StartByteMap::new(),
                stride: 8,
                pattern_len: Some(2),
                universal_start_unanchored: None,
                universal_start_anchored: None,
            };
            Self { st: start_table }
        }
    }

    let dfa = TestDFA::new();
    let mode = Anchored::Pattern(PatternID(1));
    let result = dfa.universal_start_state(mode);
}

