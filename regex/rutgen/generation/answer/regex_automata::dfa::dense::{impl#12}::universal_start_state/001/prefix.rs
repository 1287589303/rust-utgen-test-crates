// Answer 0

#[test]
fn test_universal_start_state_with_pattern() {
    struct TestDFA {
        st: StartTable<Vec<u32>>,
    }

    let test_dfa = TestDFA {
        st: StartTable {
            universal_start_unanchored: Some(StateID(0)),
            universal_start_anchored: Some(StateID(1)),
            ..Default::default()
        },
    };

    let mode = Anchored::Pattern(PatternID(0));
    let _ = test_dfa.universal_start_state(mode);
}

