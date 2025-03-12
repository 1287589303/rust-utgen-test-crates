// Answer 0

#[test]
fn test_memory_usage_none() {
    let dfa = DFA::none();
    let usage = dfa.memory_usage();
}

#[test]
fn test_memory_usage_with_engine() {
    use crate::dfa::regex::Regex;

    let regex_info = RegexInfo::new("a*b*");
    let prefilter = Some(Prefilter::default());
    let nfa = NFA::default();
    let nfarev = NFA::default();
    
    let dfa = DFA::new(&regex_info, prefilter, &nfa, &nfarev);
    let usage = dfa.memory_usage();
}

#[test]
fn test_memory_usage_with_empty_engine() {
    use crate::dfa::regex::Regex;

    let regex_info = RegexInfo::new("");
    let prefilter = Some(Prefilter::default());
    let nfa = NFA::default();
    let nfarev = NFA::default();
    
    let dfa = DFA::new(&regex_info, prefilter, &nfa, &nfarev);
    let usage = dfa.memory_usage();
}

