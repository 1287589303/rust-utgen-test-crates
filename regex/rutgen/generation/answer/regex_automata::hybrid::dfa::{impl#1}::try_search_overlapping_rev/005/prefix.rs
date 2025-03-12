// Answer 0

#[test]
fn test_try_search_overlapping_rev_error_case_1() {
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .thompson(thompson::Config::new().reverse(true))
        .build_many(&["valid_pattern"])?;
    let mut cache = dfa.create_cache();
    let input = Input::new("valid UTF-8 input");
    let mut state = OverlappingState::start();

    let result = dfa.try_search_overlapping_rev(&mut cache, &input, &mut state);
}

#[test]
fn test_try_search_overlapping_rev_none_case() {
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .thompson(thompson::Config::new().reverse(true))
        .build_many(&["another_pattern"])?;
    let mut cache = dfa.create_cache();
    let input = Input::new("no match here");
    let mut state = OverlappingState::start();

    let result = dfa.try_search_overlapping_rev(&mut cache, &input, &mut state);
}

#[test]
fn test_try_search_overlapping_rev_error_case_2() {
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .thompson(thompson::Config::new().reverse(true))
        .build_many(&["yet_another_pattern"])?;
    let mut cache = dfa.create_cache();
    let input = Input::new("more valid UTF-8 input");
    let mut state = OverlappingState::start();

    let result = dfa.try_search_overlapping_rev(&mut cache, &input, &mut state);
}

