// Answer 0

#[test]
fn test_into_nfa_with_minimum_length() {
    let mut state_builder_matches = StateBuilderMatches(vec![0; 13]);
    state_builder_matches.add_match_pattern_id(PatternID::new(1));
    let nfa = state_builder_matches.into_nfa();
}

#[test]
fn test_into_nfa_with_length_multiple_of_pattern_id_size() {
    let pattern_id_size = PatternID::SIZE;
    let length = pattern_id_size * 2;
    let mut state_builder_matches = StateBuilderMatches(vec![0; length + 5]);
    state_builder_matches.add_match_pattern_id(PatternID::new(2));
    let nfa = state_builder_matches.into_nfa();
}

#[test]
fn test_into_nfa_with_large_length() {
    let pattern_id_size = PatternID::SIZE;
    let length = pattern_id_size * 5;
    let mut state_builder_matches = StateBuilderMatches(vec![0; length + 10]);
    state_builder_matches.add_match_pattern_id(PatternID::new(3));
    let nfa = state_builder_matches.into_nfa();
}

#[test]
fn test_into_nfa_with_additional_bytes() {
    let pattern_id_size = PatternID::SIZE;
    let length = pattern_id_size * 3;
    let mut state_builder_matches = StateBuilderMatches(vec![1; length + 20]);
    state_builder_matches.add_match_pattern_id(PatternID::new(4));
    let nfa = state_builder_matches.into_nfa();
}

