// Answer 0

#[test]
fn test_find_fwd_imp_case_1() {
    let haystack: &[u8] = b"test haystack";
    let span = Span { start: 0, end: haystack.len() };
    let input = Input::new(&haystack).span(span);
    let dfa = DFA::always_match().unwrap();
    let mut cache = dfa.create_cache();
    let prefilter = Prefilter::new(MatchKind::All, &[b"test"]).unwrap();

    let result = find_fwd_imp(&dfa, &mut cache, &input, Some(&prefilter), false);
}

#[test]
fn test_find_fwd_imp_case_2() {
    let haystack: &[u8] = b"example haystack for matching";
    let span = Span { start: 0, end: haystack.len() };
    let input = Input::new(&haystack).span(span);
    let dfa = DFA::new("example").unwrap();
    let mut cache = dfa.create_cache();
    let prefilter = Prefilter::new(MatchKind::All, &[b"example"]).unwrap();

    let result = find_fwd_imp(&dfa, &mut cache, &input, Some(&prefilter), false);
}

#[test]
fn test_find_fwd_imp_case_3() {
    let haystack: &[u8] = b"find me if you can";
    let span = Span { start: 0, end: haystack.len() };
    let input = Input::new(&haystack).span(span);
    let dfa = DFA::new("find").unwrap();
    let mut cache = dfa.create_cache();
    let prefilter = Prefilter::new(MatchKind::All, &[b"find"]).unwrap();

    let result = find_fwd_imp(&dfa, &mut cache, &input, Some(&prefilter), false);
}

