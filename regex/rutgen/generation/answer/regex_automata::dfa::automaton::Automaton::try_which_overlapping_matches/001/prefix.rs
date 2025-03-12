// Answer 0

#[test]
fn test_try_which_overlapping_matches_empty_input() {
    let patterns = &["foo", "bar"];
    let dfa = DFA::builder().build_many(patterns).unwrap();
    let input = Input::new(&b""[..]);
    let mut patset = PatternSet::new(dfa.pattern_len());
    let state = OverlappingState::start();
    dfa.try_which_overlapping_matches(&input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_invalid_input() {
    let patterns = &["foo", "bar"];
    let dfa = DFA::builder().build_many(patterns).unwrap();
    let input = Input::new(&b"\xFF\xFF"[..]);
    let mut patset = PatternSet::new(dfa.pattern_len());
    let state = OverlappingState::start();
    dfa.try_which_overlapping_matches(&input, &mut patset).unwrap();
}

#[test]
fn test_try_which_overlapping_matches_full_pattern_set() {
    let patterns = &["foo", "bar"];
    let dfa = DFA::builder().build_many(patterns).unwrap();
    let input = Input::new(&b"foobar"[..]);
    let mut patset = PatternSet::new(dfa.pattern_len());
    patset.insert(PatternID(0)).unwrap();
    patset.insert(PatternID(1)).unwrap();
    let state = OverlappingState::start();
    dfa.try_which_overlapping_matches(&input, &mut patset).unwrap();
}

