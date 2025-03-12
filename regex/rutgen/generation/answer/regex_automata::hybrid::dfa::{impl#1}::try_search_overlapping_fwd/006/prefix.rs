// Answer 0

#[test]
fn test_try_search_overlapping_fwd_with_match_and_utf8() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        HalfMatch, Input, MatchKind, Cache,
    };

    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(&[r"\w+$", r"\S+$"]).unwrap();

    let mut cache = dfa.create_cache();
    let haystack = "@foo";
    let mut state = OverlappingState::start();

    // Mocking behavior for the test.
    let input = Input::new(haystack);
    dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state).unwrap();

    assert!(state.get_match().is_some());
    // further assertions can be done if needed
}

#[test]
fn test_try_search_overlapping_fwd_with_consecutive_matches() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        HalfMatch, Input, MatchKind, Cache,
    };

    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(&[r"\w+$", r"\S+$"]).unwrap();

    let mut cache = dfa.create_cache();
    let haystack = "@foo bar";
    let mut state = OverlappingState::start();

    // First search
    let input = Input::new(haystack);
    dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state).unwrap();
    assert!(state.get_match().is_some());

    // Modify state and search again
    let mut state2 = OverlappingState::start();
    dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state2).unwrap();
    assert!(state2.get_match().is_some());
}

#[test]
fn test_try_search_overlapping_fwd_with_utf8_and_valid_state() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        HalfMatch, Input, MatchKind, Cache,
    };

    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(&[r"\w{3,}", r"\S{3,}"]).unwrap();

    let mut cache = dfa.create_cache();
    let haystack = "@foobar";
    let mut state = OverlappingState::start();

    let input = Input::new(haystack);
    dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state).unwrap();

    assert!(state.get_match().is_some());
}

