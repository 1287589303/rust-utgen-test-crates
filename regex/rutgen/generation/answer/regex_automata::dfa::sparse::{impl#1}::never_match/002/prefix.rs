// Answer 0

#[test]
fn test_never_match_empty_input() {
    let dfa = sparse::DFA::never_match().unwrap();
    let input = Input::new("");
    dfa.try_search_fwd(&input).unwrap();
}

#[test]
fn test_never_match_single_character_input() {
    let dfa = sparse::DFA::never_match().unwrap();
    let input = Input::new("a");
    dfa.try_search_fwd(&input).unwrap();
}

#[test]
fn test_never_match_multi_character_input() {
    let dfa = sparse::DFA::never_match().unwrap();
    let input = Input::new("foo");
    dfa.try_search_fwd(&input).unwrap();
}

#[test]
fn test_never_match_large_input() {
    let dfa = sparse::DFA::never_match().unwrap();
    let input = Input::new(&"a".repeat(1000));
    dfa.try_search_fwd(&input).unwrap();
}

