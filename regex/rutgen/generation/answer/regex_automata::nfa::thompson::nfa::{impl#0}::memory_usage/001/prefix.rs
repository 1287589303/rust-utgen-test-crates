// Answer 0

#[test]
fn test_memory_usage_empty_nfa() {
    let nfa = NFA::never_match();
    let _ = nfa.memory_usage();
}

#[test]
fn test_memory_usage_single_state() {
    let nfa = NFA::always_match();
    let _ = nfa.memory_usage();
}

#[test]
fn test_memory_usage_single_pattern() {
    #[cfg(feature = "syntax")]
    {
        let nfa = NFA::new("a").unwrap();
        let _ = nfa.memory_usage();
    }
}

#[test]
fn test_memory_usage_complex_pattern() {
    #[cfg(feature = "syntax")]
    {
        let nfa = NFA::new(r"(a|b|c)+").unwrap();
        let _ = nfa.memory_usage();
    }
}

#[test]
fn test_memory_usage_large_states() {
    let mut builder = Builder::new();
    for _ in 0..1000 {
        builder.add_state(State::Match { pattern_id: PatternID::default() });
    }
    let nfa = builder.build().unwrap();
    let _ = nfa.memory_usage();
}

#[test]
fn test_memory_usage_group_info() {
    #[cfg(feature = "syntax")]
    {
        let nfa = NFA::new(r"(a)(b)").unwrap();
        let _ = nfa.memory_usage();
    }
}

#[test]
fn test_memory_usage_utf8_enabled() {
    #[cfg(feature = "syntax")]
    {
        let nfa = NFA::new(r"\w").unwrap();
        let _ = nfa.memory_usage();
    }
}

#[test]
fn test_memory_usage_capture_enabled() {
    #[cfg(feature = "syntax")]
    {
        let nfa = NFA::new(r"(?P<name>a)").unwrap();
        let _ = nfa.memory_usage();
    }
}

