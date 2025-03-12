// Answer 0

#[test]
fn test_universal_start_state_no() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }

    impl TestDFA {
        fn new() -> Self {
            TestDFA {
                st: StartTable {
                    table: vec![0; 8],
                    kind: StartKind::Both,
                    start_map: StartByteMap::default(),
                    stride: 1,
                    pattern_len: None,
                    universal_start_unanchored: Some(StateID(0)),
                    universal_start_anchored: None,
                },
            }
        }
    }

    let dfa = TestDFA::new();
    let mode = Anchored::No;
    let _ = dfa.universal_start_state(mode);
}

#[test]
fn test_universal_start_state_yes() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }

    impl TestDFA {
        fn new() -> Self {
            TestDFA {
                st: StartTable {
                    table: vec![0; 8],
                    kind: StartKind::Both,
                    start_map: StartByteMap::default(),
                    stride: 1,
                    pattern_len: None,
                    universal_start_unanchored: None,
                    universal_start_anchored: Some(StateID(1)),
                },
            }
        }
    }

    let dfa = TestDFA::new();
    let mode = Anchored::Yes;
    let _ = dfa.universal_start_state(mode);
}

#[test]
fn test_universal_start_state_pattern() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }

    impl TestDFA {
        fn new() -> Self {
            TestDFA {
                st: StartTable {
                    table: vec![0; 8],
                    kind: StartKind::Both,
                    start_map: StartByteMap::default(),
                    stride: 1,
                    pattern_len: None,
                    universal_start_unanchored: None,
                    universal_start_anchored: None,
                },
            }
        }
    }

    let dfa = TestDFA::new();
    let mode = Anchored::Pattern(PatternID(0));
    let _ = dfa.universal_start_state(mode);
}

