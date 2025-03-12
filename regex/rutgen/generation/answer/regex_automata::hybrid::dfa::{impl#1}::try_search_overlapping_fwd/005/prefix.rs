// Answer 0

#[test]
#[should_panic]
fn test_try_search_overlapping_fwd_with_empty_nfa_and_no_matches() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        HalfMatch, Input, MatchKind, Cache,
    };
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(&["abc", "def"]).unwrap();

    let mut cache = dfa.create_cache();
    let haystack = b"xyz";
    let mut state = OverlappingState::start();
    dfa.try_search_overlapping_fwd(&mut cache, &Input::new(haystack), &mut state).unwrap();
}

#[test]
#[should_panic]
fn test_try_search_overlapping_fwd_with_empty_nfa_multiple_patterns_no_matches() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        HalfMatch, Input, MatchKind, Cache,
    };
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(&["123", "456"]).unwrap();

    let mut cache = dfa.create_cache();
    let haystack = b"abc";
    let mut state = OverlappingState::start();
    dfa.try_search_overlapping_fwd(&mut cache, &Input::new(haystack), &mut state).unwrap();
}

