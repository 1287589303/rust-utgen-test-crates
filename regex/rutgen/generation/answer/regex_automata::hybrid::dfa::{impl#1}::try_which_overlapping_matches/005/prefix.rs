// Answer 0

#[test]
fn test_try_which_overlapping_matches_valid_patterns() {
    use regex_automata::{hybrid::dfa::DFA, Input, MatchKind, PatternSet};
    
    let patterns = &[r"foo", r"bar", r"foobar"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    
    let input = Input::new(b"foobar");
    let mut patset = PatternSet::new(dfa.pattern_len());

    let mut state = OverlappingState::start();
    let result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    
    if let Ok(Some(match_result)) = result {
        let _ = patset.try_insert(match_result.pattern());
    }
    
    assert!(!patset.is_full());
    assert!(!input.get_earliest());
}

#[test]
fn test_try_which_overlapping_matches_empty_input() {
    use regex_automata::{hybrid::dfa::DFA, Input, MatchKind, PatternSet};
    
    let patterns = &[r"foo", r"bar", r"foobar"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    
    let input = Input::new(b"");
    let mut patset = PatternSet::new(dfa.pattern_len());

    let mut state = OverlappingState::start();
    let result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    
    if let Ok(Some(match_result)) = result {
        let _ = patset.try_insert(match_result.pattern());
    }
    
    assert!(!patset.is_full());
    assert!(!input.get_earliest());
}

#[test]
fn test_try_which_overlapping_matches_large_input() {
    use regex_automata::{hybrid::dfa::DFA, Input, MatchKind, PatternSet};
    
    let patterns = &[r"foo", r"bar", r"foobar"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    
    let input = Input::new(b"foobarbaz");
    let mut patset = PatternSet::new(dfa.pattern_len());

    let mut state = OverlappingState::start();
    let result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    
    if let Ok(Some(match_result)) = result {
        let _ = patset.try_insert(match_result.pattern());
    }
    
    assert!(!patset.is_full());
    assert!(!input.get_earliest());
}

#[test]
fn test_try_which_overlapping_matches_with_full_patset() {
    use regex_automata::{hybrid::dfa::DFA, Input, MatchKind, PatternSet};
    
    let patterns = &[r"foo", r"bar", r"foobar"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    
    let input = Input::new(b"foobarfoo");
    let mut patset = PatternSet::new(dfa.pattern_len());
    
    // Fill PatternSet to full
    for pattern in patterns.iter().map(|p| dfa.pattern_id(p)) {
        let _ = patset.try_insert(pattern);
    }

    let mut state = OverlappingState::start();
    let result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    
    if let Ok(Some(match_result)) = result {
        let _ = patset.try_insert(match_result.pattern());
    }
    
    assert!(patset.is_full());
    assert!(!input.get_earliest());
}

#[test]
#[should_panic]
fn test_try_which_overlapping_matches_err_case() {
    use regex_automata::{hybrid::dfa::DFA, Input, MatchKind, PatternSet};
    
    let patterns = &[r"xyz", r"abc"];
    let dfa = DFA::builder()
        .configure(DFA::config().match_kind(MatchKind::All))
        .build_many(patterns).unwrap();
    
    let mut cache = dfa.create_cache();
    
    let input = Input::new(b"");
    let mut patset = PatternSet::new(dfa.pattern_len());

    let mut state = OverlappingState::start();
    let _result = dfa.try_search_overlapping_fwd(&mut cache, &input, &mut state);
    
    if let Ok(Some(match_result)) = _result {
        let _ = patset.try_insert(match_result.pattern());
    }
    
    // This will panic as the state should not find a match
}

