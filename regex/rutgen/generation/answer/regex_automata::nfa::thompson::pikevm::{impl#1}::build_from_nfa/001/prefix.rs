// Answer 0

#[test]
fn test_build_from_nfa_with_empty_look_set() {
    let builder = Builder::new();
    let nfa = NFA::always_match();
    let result = builder.build_from_nfa(nfa);
}

#[test]
fn test_build_from_nfa_with_non_empty_look_set() {
    let builder = Builder::new();
    let mut nfa = NFA::always_match();
    let look_set = LookSet::singleton(Look::Word);
    nfa.0.look_set_any = look_set;
    let result = builder.build_from_nfa(nfa);
}

#[test]
fn test_build_from_nfa_with_word_boundary_error() {
    let builder = Builder::new();
    let mut nfa = NFA::always_match();
    let look_set = LookSet::full();
    nfa.0.look_set_any = look_set;
    let result = builder.build_from_nfa(nfa);
}

#[test]
fn test_build_from_nfa_with_invalid_look_set() {
    let builder = Builder::new();
    let mut nfa = NFA::always_match();
    let look_set = LookSet::empty();
    nfa.0.look_set_any = look_set;
    let result = builder.build_from_nfa(nfa);
}

