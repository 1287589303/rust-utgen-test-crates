// Answer 0

#[test]
fn test_always_match_empty_input() {
    let dfa = regex_automata::dfa::dense::OwnedDFA::always_match().unwrap();
    let input = regex_automata::util::Input::new("");
    let _ = dfa.try_search_fwd(&input).unwrap();
}

#[test]
fn test_always_match_non_empty_input() {
    let dfa = regex_automata::dfa::dense::OwnedDFA::always_match().unwrap();
    let input = regex_automata::util::Input::new("foo");
    let _ = dfa.try_search_fwd(&input).unwrap();
}

