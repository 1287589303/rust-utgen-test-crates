// Answer 0

#[test]
fn test_build_from_nfa_with_empty_nfa() {
    let nfa = NFA::always_match();
    let mut builder = dense::Builder::new();
    let config = Config::new()
        .unicode_word_boundary(false)
        .byte_classes(false);
    builder.configure(config);
    let result = builder.build_from_nfa(&nfa);
}

#[test]
fn test_build_from_nfa_with_invalid_pattern_length() {
    let nfa = NFA::never_match();
    let mut builder = dense::Builder::new();
    let config = Config::new()
        .unicode_word_boundary(false)
        .byte_classes(false);
    builder.configure(config);
    let result = builder.build_from_nfa(&nfa);
}

#[test]
fn test_build_from_nfa_with_starts_both() {
    let nfa = NFA::compiler().build(r"[0-9]+").unwrap();
    let mut builder = dense::Builder::new();
    let config = Config::new()
        .unicode_word_boundary(false)
        .byte_classes(false)
        .start_kind(StartKind::Both)
        .starts_for_each_pattern(true)
        .prefilter(None);
    builder.configure(config);
    let result = builder.build_from_nfa(&nfa);
}

#[test]
fn test_build_from_nfa_with_starts_anchored() {
    let nfa = NFA::compiler().build(r"[a-z]+").unwrap();
    let mut builder = dense::Builder::new();
    let config = Config::new()
        .unicode_word_boundary(false)
        .byte_classes(false)
        .start_kind(StartKind::Anchored)
        .starts_for_each_pattern(false)
        .prefilter(None);
    builder.configure(config);
    let result = builder.build_from_nfa(&nfa);
}

#[test]
fn test_build_from_nfa_with_starts_unanchored() {
    let nfa = NFA::compiler().build(r"abc").unwrap();
    let mut builder = dense::Builder::new();
    let config = Config::new()
        .unicode_word_boundary(false)
        .byte_classes(false)
        .start_kind(StartKind::Unanchored)
        .starts_for_each_pattern(true)
        .prefilter(None);
    builder.configure(config);
    let result = builder.build_from_nfa(&nfa);
}

