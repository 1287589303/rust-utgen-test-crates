// Answer 0

#[test]
fn test_new_empty_matches() {
    use alloc::collections::BTreeMap;

    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
    let pattern_len: usize = 0;

    let result = MatchStates::new(&matches, pattern_len);
}

#[test]
fn test_new_empty_matches_non_zero_pattern_len() {
    use alloc::collections::BTreeMap;

    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
    let pattern_len: usize = 1;

    let result = MatchStates::new(&matches, pattern_len);
}

