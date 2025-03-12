// Answer 0

#[test]
fn test_all_group_len_empty_string() {
    let nfa = NFA::new("")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_single_character() {
    let nfa = NFA::new("a")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_single_explicit_group() {
    let nfa = NFA::new("(a)")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_multiple_explicit_groups() {
    let nfa = NFA::new("(a)(b)(c)")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_multiple_explicit_and_implicit_groups() {
    let nfa = NFA::new("(a)(b)")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_implicit_group_with_no_explicit_groups() {
    let nfa = NFA::new("abc")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_disabled_capturing() {
    let nfa = NFA::compiler()
        .configure(NFA::config().which_captures(WhichCaptures::None))
        .build("abc")?;
    nfa.group_info().all_group_len();
}

#[test]
fn test_all_group_len_disabled_capturing_with_explicit_groups() {
    let nfa = NFA::compiler()
        .configure(NFA::config().which_captures(WhichCaptures::None))
        .build("(a)(b)(c)")?;
    nfa.group_info().all_group_len();
}

