// Answer 0

#[test]
fn test_build_from_simple_nfa() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"[a-z]+")
        .unwrap();
    let builder = Builder::new();
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_from_complex_nfa() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"(abc|def|ghi)+")
        .unwrap();
    let builder = Builder::new();
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_from_multiple_patterns_nfa() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"(foo|bar|baz)+")
        .unwrap();
    let builder = Builder::new();
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_nfa_with_pattern_id_limit() {
    let pattern: String = (0..200).map(|i| format!("[a-z]{i}")).collect::<Vec<String>>().join("|");
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(&pattern)
        .unwrap();
    let builder = Builder::new();
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_nfa_with_byte_classes_disabled() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"[a-z]+")
        .unwrap();
    let mut builder = Builder::new();
    builder.configure(Config { byte_classes: Some(false), ..Default::default() });
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_nfa_with_different_match_kind() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"hello|world")
        .unwrap();
    let mut builder = Builder::new();
    builder.configure(Config { match_kind: Some(MatchKind::Full), ..Default::default() });
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_nfa_with_explicit_slots() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"(\d{1,3})-(\d{1,3})")
        .unwrap();
    let builder = Builder::new();
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

#[test]
fn test_build_with_empty_nfa() {
    let nfa = NFA::compiler()
        .configure(NFA::config().shrink(true))
        .build(r"") // Empty pattern
        .unwrap();
    let builder = Builder::new();
    let dfa = builder.build_from_nfa(nfa).unwrap();
}

