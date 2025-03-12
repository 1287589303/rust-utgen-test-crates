// Answer 0

#[test]
fn test_start_pattern_valid_0() {
    let nfa = NFA::new_many(&["^a", "^b"]).unwrap();
    let pid = PatternID::must(0);
    let _result = nfa.start_pattern(pid);
}

#[test]
fn test_start_pattern_valid_1() {
    let nfa = NFA::new_many(&["^a", "^b"]).unwrap();
    let pid = PatternID::must(1);
    let _result = nfa.start_pattern(pid);
}

#[test]
fn test_start_pattern_invalid() {
    let nfa = NFA::new_many(&["^a", "^b"]).unwrap();
    let pid = PatternID::must(2);
    let _result = nfa.start_pattern(pid);
}

