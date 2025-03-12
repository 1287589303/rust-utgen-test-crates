// Answer 0

#[test]
fn test_reverse_dfa_engine_new_dfa_disabled() {
    let config = Config::new().dfa(false);
    let regex_info = RegexInfo::new(config, &[]);
    let nfa = NFA::never_match();
    
    let result = ReverseDFAEngine::new(&regex_info, &nfa);
}

#[test]
fn test_reverse_dfa_engine_new_dfa_disabled_with_state_limit() {
    let config = Config::new().dfa(false).dfa_state_limit(Some(10));
    let regex_info = RegexInfo::new(config, &[]);
    let nfa = NFA::never_match();

    let result = ReverseDFAEngine::new(&regex_info, &nfa);
}

