// Answer 0

#[test]
fn test_has_capture_with_single_capture_group() {
    let nfa = NFA::new("(a)").unwrap();
    nfa.has_capture();
}

#[test]
fn test_has_capture_with_anonymous_capture() {
    let nfa = NFA::new("a").unwrap();
    nfa.has_capture();
}

#[test]
fn test_has_capture_no_capture_groups() {
    let nfa = NFA::compiler()
        .configure(NFA::config().which_captures(WhichCaptures::None))
        .build("(a)").unwrap();
    nfa.has_capture();
}

#[test]
fn test_has_capture_with_multiple_capture_groups() {
    let nfa = NFA::new("((a)(b))").unwrap();
    nfa.has_capture();
}

#[test]
fn test_has_capture_with_empty_pattern() {
    let nfa = NFA::new("").unwrap();
    nfa.has_capture();
}

#[test]
fn test_has_capture_with_complex_nested_capture_groups() {
    let nfa = NFA::new("((a(bc)d)ef)").unwrap();
    nfa.has_capture();
}

