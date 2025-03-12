// Answer 0

#[test]
fn test_find_rev_input_done_case1() {
    let haystack: &[u8] = b"";
    let span = Span::new(0, 0);
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(false);
    let mut cache = Cache::default();
    let dfa = DFA::default();
    let result = find_rev(&dfa, &mut cache, &input);
}

#[test]
fn test_find_rev_input_done_case2() {
    let haystack: &[u8] = b"example";
    let span = Span::new(3, 3);
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::Yes)
        .earliest(true);
    let mut cache = Cache::default();
    let dfa = DFA::default();
    let result = find_rev(&dfa, &mut cache, &input);
}

#[test]
fn test_find_rev_input_done_case3() {
    let haystack: &[u8] = b"another test";
    let span = Span::new(5, 5);
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(true);
    let mut cache = Cache::default();
    let dfa = DFA::default();
    let result = find_rev(&dfa, &mut cache, &input);
}

#[test]
fn test_find_rev_input_done_case4() {
    let haystack: &[u8] = b"abc";
    let span = Span::new(2, 2);
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::Yes)
        .earliest(false);
    let mut cache = Cache::default();
    let dfa = DFA::default();
    let result = find_rev(&dfa, &mut cache, &input);
}

