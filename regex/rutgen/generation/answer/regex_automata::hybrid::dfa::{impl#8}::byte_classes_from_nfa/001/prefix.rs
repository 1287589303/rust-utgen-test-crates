// Answer 0

#[test]
fn test_byte_classes_from_nfa_case_1() {
    let config = Config::default().byte_classes(true);
    let nfa = NFA::always_match();
    let quit = ByteSet::empty();
    let result = config.byte_classes_from_nfa(&nfa, &quit);
}

#[test]
fn test_byte_classes_from_nfa_case_2() {
    let config = Config::default().byte_classes(true);
    let nfa = NFA::new(".*").expect("Failed to create NFA");
    let quit = ByteSet::empty();
    let result = config.byte_classes_from_nfa(&nfa, &quit);
}

#[test]
fn test_byte_classes_from_nfa_case_3() {
    let config = Config::default().byte_classes(true);
    let nfa = NFA::new_many(&["abc", "def"]).expect("Failed to create NFA");
    let quit = ByteSet::empty();
    let result = config.byte_classes_from_nfa(&nfa, &quit);
}

#[test]
fn test_byte_classes_from_nfa_case_4() {
    let config = Config::default().byte_classes(true);
    let nfa = NFA::never_match();
    let quit = ByteSet::empty();
    let result = config.byte_classes_from_nfa(&nfa, &quit);
}

