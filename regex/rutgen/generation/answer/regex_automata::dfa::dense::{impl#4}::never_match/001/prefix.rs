// Answer 0

#[test]
fn test_never_match_empty_input() {
    let dfa = dense::never_match().unwrap();
    let input = Input::new("");
    dfa.try_search_fwd(&input).unwrap();
}

#[test]
fn test_never_match_non_empty_input() {
    let dfa = dense::never_match().unwrap();
    let input = Input::new("foo");
    dfa.try_search_fwd(&input).unwrap();
}

