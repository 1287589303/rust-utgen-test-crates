// Answer 0

#[test]
fn test_group_info_with_valid_pattern() {
    let nfa = regex_automata::nfa::thompson::NFA::new(r"(a)(?P<foo>b)(c)(d)(?P<bar>e)").unwrap();
    let group_info = nfa.group_info();
}

#[test]
fn test_group_info_with_invalid_pattern_id() {
    let nfa = regex_automata::nfa::thompson::NFA::new(r"(a)(?P<foo>b)(c)(d)(?P<bar>e)").unwrap();
    let group_info = nfa.group_info();
    let invalid_id = regex_automata::util::primitives::PatternID::must(999);
    let count = group_info.pattern_names(invalid_id).count();
}

#[test]
fn test_group_info_with_empty_pattern() {
    let nfa = regex_automata::nfa::thompson::NFA::new(r"()").unwrap();
    let group_info = nfa.group_info();
}

#[test]
fn test_group_info_with_single_named_capture() {
    let nfa = regex_automata::nfa::thompson::NFA::new(r"(?P<single>abc)").unwrap();
    let group_info = nfa.group_info();
}

#[test]
fn test_group_info_with_multiple_named_and_unnamed_captures() {
    let nfa = regex_automata::nfa::thompson::NFA::new(r"(?P<first>a)(?P<second>b)(c)").unwrap();
    let group_info = nfa.group_info();
}

