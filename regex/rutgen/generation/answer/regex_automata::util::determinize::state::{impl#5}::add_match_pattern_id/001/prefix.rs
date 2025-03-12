// Answer 0

#[test]
fn test_add_match_pattern_id_zero() {
    let mut state_builder_matches = StateBuilderMatches(vec![]);
    let pattern_id_zero = PatternID::ZERO;
    state_builder_matches.add_match_pattern_id(pattern_id_zero);
}

#[test]
fn test_add_match_pattern_id_max() {
    let mut state_builder_matches = StateBuilderMatches(vec![]);
    let pattern_id_max = PatternID(StateID::MAX);
    state_builder_matches.add_match_pattern_id(pattern_id_max);
}

#[test]
fn test_add_match_pattern_id_typical() {
    let mut state_builder_matches = StateBuilderMatches(vec![]);
    let pattern_id_typical = PatternID(StateID(1));
    state_builder_matches.add_match_pattern_id(pattern_id_typical);
}

