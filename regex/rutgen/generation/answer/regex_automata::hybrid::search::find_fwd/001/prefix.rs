// Answer 0

#[test]
fn test_find_fwd_input_done_empty_haystack() {
    let haystack = b"";
    let span = Span::from(0..0);
    let input = Input::new(&haystack).span(span).anchored(Anchored::No).earliest(false);
    let dfa = DFA::always_match().unwrap();
    let mut cache = dfa.create_cache();
    let result = find_fwd(&dfa, &mut cache, &input);
}

#[test]
fn test_find_fwd_input_done_haystack_short() {
    let haystack = b"abc";
    let span = Span::from(1..0);
    let input = Input::new(&haystack).span(span).anchored(Anchored::Yes).earliest(true);
    let dfa = DFA::always_match().unwrap();
    let mut cache = dfa.create_cache();
    let result = find_fwd(&dfa, &mut cache, &input);
}

#[test]
fn test_find_fwd_input_done_haystack_with_space() {
    let haystack = b" ";
    let span = Span::from(0..0);
    let input = Input::new(&haystack).span(span).anchored(Anchored::Pattern(PatternID(1))).earliest(false);
    let dfa = DFA::never_match().unwrap();
    let mut cache = dfa.create_cache();
    let result = find_fwd(&dfa, &mut cache, &input);
}

#[test]
fn test_find_fwd_input_done_k_long_haystack() {
    let haystack = b"abcdefg";
    let span = Span::from(3..3);
    let input = Input::new(&haystack).span(span).anchored(Anchored::No).earliest(true);
    let dfa = DFA::new("abc").unwrap();
    let mut cache = dfa.create_cache();
    let result = find_fwd(&dfa, &mut cache, &input);
}

#[test]
fn test_find_fwd_input_done_haystack_single_char() {
    let haystack = b"a";
    let span = Span::from(1..1);
    let input = Input::new(&haystack).span(span).anchored(Anchored::Yes).earliest(false);
    let dfa = DFA::new("a").unwrap();
    let mut cache = dfa.create_cache();
    let result = find_fwd(&dfa, &mut cache, &input);
}

