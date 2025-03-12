// Answer 0

#[test]
fn test_validate_pattern_id_error() {
    struct TestDFA {
        special: Special,
        pattern_ids: Vec<PatternID>,
    }

    let k = 3; // Example pattern length
    let pattern_ids = vec![PatternID(0), PatternID(1), PatternID(2)];
    let special = Special {
        max_match: StateID(2),
        min_match: StateID(0),
        ..Default::default()
    };

    let dfa = TestDFA {
        special,
        pattern_ids,
    };

    let match_states = MatchStates {
        slices: vec![k as u32, 1], // Match state whose start offset equals pattern_ids.len()
        pattern_ids: vec![PatternID(0), PatternID(1), PatternID(2)],
        pattern_len: k,
    };

    match match_states.validate(&dfa) {
        Err(_) => {}
        Ok(_) => panic!("Expected validation to fail due to invalid pattern ID"),
    }
}

#[test]
fn test_validate_valid_input() {
    struct TestDFA {
        special: Special,
        pattern_ids: Vec<PatternID>,
    }

    let k = 3; // Example pattern length
    let pattern_ids = vec![PatternID(0), PatternID(1), PatternID(2)];
    let special = Special {
        max_match: StateID(2),
        min_match: StateID(0),
        ..Default::default()
    };

    let dfa = TestDFA {
        special,
        pattern_ids,
    };

    let match_states = MatchStates {
        slices: vec![1, 1, 2, 1], // Valid slices leading to valid patterns
        pattern_ids: vec![PatternID(0), PatternID(1), PatternID(2)],
        pattern_len: k,
    };

    match match_states.validate(&dfa) {
        Ok(_) => {}
        Err(_) => panic!("Expected validation to succeed"),
    }
}

