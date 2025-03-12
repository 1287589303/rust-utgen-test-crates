// Answer 0

#[test]
fn test_reverse_dfa_engine_new_returns_none_due_to_dfa_disabled() {
    let config = Config::new().dfa(false);
    let info = RegexInfo::new(config, &[]);
    let nfa = NFA::never_match();
    let engine = ReverseDFAEngine::new(&info, &nfa);
    assert_eq!(engine, None);
}

#[test]
fn test_reverse_dfa_engine_new_returns_none_due_to_state_limit_exceeding() {
    let state_limit_value = 5;
    let config = Config::new()
        .dfa(true)
        .dfa_state_limit(Some(state_limit_value));
    let info = RegexInfo::new(config, &[]);
    
    let states = vec![State::Match { pattern_id: 0 }; 6]; // 6 states, exceeding the limit
    let nfa = NFA::new(states);

    let engine = ReverseDFAEngine::new(&info, &nfa);
    assert_eq!(engine, None);
}

#[test]
fn test_reverse_dfa_engine_new_returns_none_due_to_dfa_build_failure() {
    let state_limit_value = 3;
    let config = Config::new()
        .dfa(true)
        .dfa_state_limit(Some(state_limit_value))
        .dfa_size_limit(Some(128)); // Arbitrary value to enforce certain limits

    let info = RegexInfo::new(config, &[]);
    let states = vec![State::Match { pattern_id: 0 }; 4]; // 4 states, exceeds dfa_state_limit
    let nfa = NFA::new(states);

    let engine = ReverseDFAEngine::new(&info, &nfa);
    assert_eq!(engine, None);
}

