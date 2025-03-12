// Answer 0

#[test]
fn test_new_with_map_single_pattern() {
    let mut matches = BTreeMap::new();
    let state_id = StateID(SmallIndex::new(0));
    let pattern_id = PatternID(SmallIndex::new(0));
    matches.insert(state_id, vec![pattern_id]);
    let match_states = MatchStates::empty(1);
    let _ = match_states.new_with_map(&matches);
}

#[test]
fn test_new_with_map_multiple_patterns() {
    let mut matches = BTreeMap::new();
    let state_id_1 = StateID(SmallIndex::new(0));
    let state_id_2 = StateID(SmallIndex::new(1));
    let pattern_id_1 = PatternID(SmallIndex::new(0));
    let pattern_id_2 = PatternID(SmallIndex::new(1));
    matches.insert(state_id_1, vec![pattern_id_1]);
    matches.insert(state_id_2, vec![pattern_id_2, pattern_id_1]);
    let match_states = MatchStates::empty(2);
    let _ = match_states.new_with_map(&matches);
}

#[test]
fn test_new_with_map_empty_pattern_len() {
    let mut matches = BTreeMap::new();
    let state_id = StateID(SmallIndex::new(0));
    let pattern_id = PatternID(SmallIndex::new(0));
    matches.insert(state_id, vec![pattern_id]);
    let match_states = MatchStates::empty(0);
    let _ = match_states.new_with_map(&matches);
}

#[test]
fn test_new_with_map_maximum_patterns() {
    let mut matches = BTreeMap::new();
    let state_id = StateID(SmallIndex::new(0));
    let pattern_ids: Vec<PatternID> = (0..10).map(|i| PatternID(SmallIndex::new(i))).collect();
    matches.insert(state_id, pattern_ids.clone());
    let match_states = MatchStates::empty(pattern_ids.len());
    let _ = match_states.new_with_map(&matches);
}

