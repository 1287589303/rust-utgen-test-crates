// Answer 0

#[test]
fn test_never_match_empty_input() {
    let dfa = DFA::never_match().unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new("");
    let _result = dfa.try_search_fwd(&mut cache, &input).unwrap();
}

#[test]
fn test_never_match_non_empty_input() {
    let dfa = DFA::never_match().unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new("foo");
    let _result = dfa.try_search_fwd(&mut cache, &input).unwrap();
}

#[test]
fn test_never_match_with_cache() {
    let dfa = DFA::never_match().unwrap();
    let mut cache = dfa.create_cache();
    let empty_input = Input::new("");
    let non_empty_input = Input::new("foo");
    let _result_empty = dfa.try_search_fwd(&mut cache, &empty_input).unwrap();
    let _result_non_empty = dfa.try_search_fwd(&mut cache, &non_empty_input).unwrap();
}

