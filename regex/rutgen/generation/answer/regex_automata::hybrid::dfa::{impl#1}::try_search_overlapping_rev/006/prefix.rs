// Answer 0

#[test]
fn test_try_search_overlapping_rev_with_utf8empty_true() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        nfa::thompson,
        HalfMatch, Input, MatchKind,
    };
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .thompson(thompson::Config::new().reverse(true).utf8(true))
        .build_many(&[r"abc", r"b", r"c", r"a"])?;
    let mut cache = dfa.create_cache();
    
    let input = Input::new("abc");
    let mut state = OverlappingState::start();
    let mut matches = vec![];
    loop {
        dfa.try_search_overlapping_rev(&mut cache, &input, &mut state)?;
        match state.get_match() {
            None => break,
            Some(hm) => matches.push(hm),
        }
    }
}

#[test]
fn test_try_search_overlapping_rev_with_utf8empty_false() {
    use regex_automata::{
        hybrid::dfa::{DFA, OverlappingState},
        nfa::thompson,
        HalfMatch, Input, MatchKind,
    };
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .thompson(thompson::Config::new().reverse(true).utf8(false))
        .build_many(&[r"abc", r"b", r"c", r"a"])?;
    let mut cache = dfa.create_cache();
    
    let input = Input::new("abc");
    let mut state = OverlappingState::start();
    let mut matches = vec![];
    loop {
        dfa.try_search_overlapping_rev(&mut cache, &input, &mut state)?;
        match state.get_match() {
            None => break,
            Some(hm) => matches.push(hm),
        }
    }
}

