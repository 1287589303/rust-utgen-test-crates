// Answer 0

#[test]
fn test_pattern_id_valid_case() {
    let slices = vec![0, 1, 2, 3]; // Assume 2 match states with 2 pattern IDs each.
    let pattern_ids = vec![
        PatternID(0.into()), PatternID(1.into()), 
        PatternID(2.into()), PatternID(3.into())
    ];
    let match_states = MatchStates {
        slices: slices.clone(),
        pattern_ids: pattern_ids.clone(),
        pattern_len: 2, // Each match state having 2 patterns
    };

    let result = match_states.pattern_id(0, 1); // Valid state_index = 0, match_index = 1
    let expected = pattern_ids[1]; // The expected PatternID
}

#[test]
fn test_pattern_id_boundary_case_first_index() {
    let slices = vec![0, 1]; // One match state with one pattern ID
    let pattern_ids = vec![PatternID(0.into())];
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 1,
    };

    let result = match_states.pattern_id(0, 0); // Valid case for first match index
}

#[test]
fn test_pattern_id_boundary_case_last_index() {
    let slices = vec![0, 2]; // One match state with two pattern IDs
    let pattern_ids = vec![PatternID(0.into()), PatternID(1.into())];
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 2,
    };

    let result = match_states.pattern_id(0, 1); // Valid case for last match index
}

#[test]
#[should_panic]
fn test_pattern_id_invalid_state_index() {
    let slices = vec![0, 1]; // One match state with one pattern ID
    let pattern_ids = vec![PatternID(0.into())];
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 1,
    };

    let _ = match_states.pattern_id(1, 0); // Invalid state_index
}

#[test]
#[should_panic]
fn test_pattern_id_invalid_match_index() {
    let slices = vec![0, 2]; // One match state with two pattern IDs
    let pattern_ids = vec![PatternID(0.into()), PatternID(1.into())];
    let match_states = MatchStates {
        slices,
        pattern_ids,
        pattern_len: 2,
    };

    let _ = match_states.pattern_id(0, 2); // Invalid match_index
}

