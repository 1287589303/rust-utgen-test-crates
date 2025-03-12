// Answer 0

#[test]
fn test_never_match_empty_input() {
    let nfa = regex_automata::nfa::thompson::NFA::never_match();
    let re = regex_automata::nfa::pikevm::PikeVM::new_from_nfa(nfa).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.captures(&mut cache, b"", &mut caps);
}

#[test]
fn test_never_match_non_empty_input() {
    let nfa = regex_automata::nfa::thompson::NFA::never_match();
    let re = regex_automata::nfa::pikevm::PikeVM::new_from_nfa(nfa).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    re.captures(&mut cache, b"foo", &mut caps);
}

