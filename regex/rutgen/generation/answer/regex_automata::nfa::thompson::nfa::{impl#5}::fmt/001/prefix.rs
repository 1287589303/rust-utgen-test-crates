// Answer 0

#[test]
fn test_state_match_formatting_valid_pattern_id() {
    let valid_pattern_id = PatternID(SmallIndex::new_unchecked(0)); // Test with the lowest valid PatternID value
    let state = State::Match { pattern_id: valid_pattern_id };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_match_formatting_middle_pattern_id() {
    let middle_pattern_id = PatternID(SmallIndex::new_unchecked(SmallIndex::MAX.as_usize() / 2)); // Test with a middle value
    let state = State::Match { pattern_id: middle_pattern_id };
    let _ = format!("{:?}", state);
}

#[test]
fn test_state_match_formatting_max_pattern_id() {
    let max_pattern_id = PatternID(SmallIndex::MAX); // Test with the maximum valid PatternID value
    let state = State::Match { pattern_id: max_pattern_id };
    let _ = format!("{:?}", state);
}

