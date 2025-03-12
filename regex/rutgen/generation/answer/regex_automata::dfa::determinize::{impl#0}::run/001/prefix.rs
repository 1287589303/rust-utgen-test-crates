// Answer 0

#[test]
fn test_run_with_valid_nfa_and_default_config() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let config = Config::new();
    let _ = config.run(&nfa, &mut dfa);
}

#[test]
fn test_run_with_match_kind_all() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let mut config = Config::new().match_kind(MatchKind::All);
    let _ = config.run(&nfa, &mut dfa);
}

#[test]
fn test_run_with_match_kind_leftmost_first() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let mut config = Config::new().match_kind(MatchKind::LeftmostFirst);
    let _ = config.run(&nfa, &mut dfa);
}

#[test]
fn test_run_with_quit_byte_set() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let quit = ByteSet::from_ranges(&[(0, 127)]);
    let mut config = Config::new().quit(quit);
    let _ = config.run(&nfa, &mut dfa);
}

#[test]
fn test_run_with_dfa_size_limit() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let mut config = Config::new().dfa_size_limit(Some(1024));
    let _ = config.run(&nfa, &mut dfa);
}

#[test]
fn test_run_with_determinize_size_limit() {
    let nfa = NFA::always_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let mut config = Config::new().determinize_size_limit(Some(1024));
    let _ = config.run(&nfa, &mut dfa);
}

#[test]
fn test_run_with_empty_nfa() {
    let nfa = NFA::never_match();
    let mut dfa = dense::OwnedDFA::new(nfa.byte_classes().len(), nfa.pattern_len()).unwrap();
    let mut config = Config::new();
    let _ = config.run(&nfa, &mut dfa);
}

