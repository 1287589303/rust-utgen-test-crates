// Answer 0

#[test]
fn test_group_len_with_explicit_groups() {
    let nfa = NFA::new(r"(a)(b)(c)").unwrap();
    let result = nfa.group_info().group_len(PatternID::ZERO);
}

#[test]
fn test_group_len_with_implicit_group() {
    let nfa = NFA::new(r"abc").unwrap();
    let result = nfa.group_info().group_len(PatternID::ZERO);
}

#[test]
fn test_group_len_with_disabled_capturing() {
    let nfa = NFA::compiler()
        .configure(NFA::config().which_captures(WhichCaptures::None))
        .build(r"abc")
        .unwrap();
    let result = nfa.group_info().group_len(PatternID::ZERO);
}

#[test]
fn test_group_len_with_disabled_capturing_explicit_groups() {
    let nfa = NFA::compiler()
        .configure(NFA::config().which_captures(WhichCaptures::None))
        .build(r"(a)(b)(c)")
        .unwrap();
    let result = nfa.group_info().group_len(PatternID::ZERO);
}

