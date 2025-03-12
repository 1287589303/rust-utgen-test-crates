// Answer 0

#[test]
fn test_one_pass_engine_new_case_1() {
    let config = Config::new().onepass(true);
    let regex_info = RegexInfo::new(config, &[]);
    let nfa = NFA::default();
    let engine = OnePassEngine::new(&regex_info, &nfa);
}

#[test]
fn test_one_pass_engine_new_case_2() {
    let config = Config::new().onepass(true);
    let regex_info = RegexInfo::new(config, &[]);
    let nfa = NFA::default();
    let engine = OnePassEngine::new(&regex_info, &nfa);
}

#[test]
fn test_one_pass_engine_new_case_3() {
    let config = Config::new().onepass(true);
    let regex_info = RegexInfo::new(config, &[]);
    let nfa = NFA::default();
    let engine = OnePassEngine::new(&regex_info, &nfa);
}

