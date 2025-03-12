// Answer 0

#[test]
fn test_find_fwd_imp_success() {
    let haystack: &[u8] = b"abcdef";
    let pattern = "abc";
    let dfa = DFA::new(pattern).unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 6 })
        .anchored(Anchored::Unanchored)
        .earliest(true);

    let prefilter = Prefilter::new(MatchKind::Prefix, &[b"abc"]).unwrap();
    let pre = Some(&prefilter);
    
    let result = find_fwd_imp(&dfa, &mut cache, &input, pre, true);
    
    assert!(result.is_ok());
}

#[test]
fn test_find_fwd_imp_universal_start_disabled() {
    let haystack: &[u8] = b"xyzabc";
    let pattern = "abc";
    let dfa = DFA::new(pattern).unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 6 })
        .anchored(Anchored::Unanchored)
        .earliest(true);

    let prefilter = Prefilter::new(MatchKind::Prefix, &[b"abc"]).unwrap();
    let pre = Some(&prefilter);

    let result = find_fwd_imp(&dfa, &mut cache, &input, pre, true);

    assert!(result.is_ok());
}

#[test]
fn test_find_fwd_imp_match_found() {
    let haystack: &[u8] = b"hello worldabc";
    let pattern = "abc";
    let dfa = DFA::new(pattern).unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 16 })
        .anchored(Anchored::Unanchored)
        .earliest(true);

    let prefilter = Prefilter::new(MatchKind::Prefix, &[b"wo"]).unwrap();
    let pre = Some(&prefilter);

    let result = find_fwd_imp(&dfa, &mut cache, &input, pre, true);

    assert!(result.is_ok());
}

#[test]
fn test_find_fwd_imp_earliest_true() {
    let haystack: &[u8] = b"abcdefabc";
    let pattern = "abc";
    let dfa = DFA::new(pattern).unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 10 })
        .anchored(Anchored::Unanchored)
        .earliest(true);

    let prefilter = Prefilter::new(MatchKind::Prefix, &[b"abc"]).unwrap();
    let pre = Some(&prefilter);

    let result = find_fwd_imp(&dfa, &mut cache, &input, pre, true);

    assert!(result.is_ok());
}

#[test]
fn test_find_fwd_imp_next_state_ok() {
    let haystack: &[u8] = b"abxyzabc";
    let pattern = "abc";
    let dfa = DFA::new(pattern).unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 8 })
        .anchored(Anchored::Unanchored)
        .earliest(true);

    let prefilter = Prefilter::new(MatchKind::Prefix, &[b"ab"]).unwrap();
    let pre = Some(&prefilter);

    let result = find_fwd_imp(&dfa, &mut cache, &input, pre, true);

    assert!(result.is_ok());
}

