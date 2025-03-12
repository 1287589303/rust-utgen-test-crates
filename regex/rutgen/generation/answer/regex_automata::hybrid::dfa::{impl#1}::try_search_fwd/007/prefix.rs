// Answer 0

#[test]
fn test_try_search_fwd_match_found() {
    let dfa = DFA::new("foo[0-9]+").expect("Failed to create DFA");
    let mut cache = dfa.create_cache();
    let input = Input::new("foo12345");
    let result = dfa.try_search_fwd(&mut cache, &input).expect("Search failed");
}

#[test]
fn test_try_search_fwd_no_match() {
    let dfa = DFA::new("abc").expect("Failed to create DFA");
    let mut cache = dfa.create_cache();
    let input = Input::new("xyz");
    let result = dfa.try_search_fwd(&mut cache, &input).expect("Search failed");
}

#[test]
fn test_try_search_fwd_empty_input() {
    let dfa = DFA::new("abc").expect("Failed to create DFA");
    let mut cache = dfa.create_cache();
    let input = Input::new("");
    let result = dfa.try_search_fwd(&mut cache, &input).expect("Search failed");
}

#[test]
fn test_try_search_fwd_utf8_valid_match() {
    let dfa = DFA::new("abc").expect("Failed to create DFA");
    let mut cache = dfa.create_cache();
    let input = Input::new("abcde");
    let result = dfa.try_search_fwd(&mut cache, &input).expect("Search failed");
}

#[test]
fn test_try_search_fwd_multiple_patterns() {
    let dfa = DFA::builder()
        .configure(DFA::config().starts_for_each_pattern(true))
        .build_many(&["[a-z0-9]{6}", "[a-z][a-z0-9]{5}"]).expect("Failed to create multi-DFA");
    let mut cache = dfa.create_cache();
    let input = Input::new("foo123");
    let result = dfa.try_search_fwd(&mut cache, &input).expect("Search failed");
}

