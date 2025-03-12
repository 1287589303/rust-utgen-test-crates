// Answer 0

#[test]
fn test_minimize_with_single_state() {
    let mut dfa: regex_automata::OwnedDFA = regex_automata::OwnedDFA::default();
    dfa.add_empty_state().unwrap();
    dfa.minimize();
}

#[test]
fn test_minimize_with_multiple_states() {
    let mut dfa: regex_automata::OwnedDFA = regex_automata::OwnedDFA::default();
    let state1 = dfa.add_empty_state().unwrap();
    let state2 = dfa.add_empty_state().unwrap();
    dfa.set_transition(state1, 1, state2);
    dfa.set_transition(state2, 2, state1);
    dfa.minimize();
}

#[test]
fn test_minimize_with_pattern_map() {
    let mut dfa: regex_automata::OwnedDFA = regex_automata::OwnedDFA::default();
    let state1 = dfa.add_empty_state().unwrap();
    let state2 = dfa.add_empty_state().unwrap();
    dfa.set_transition(state1, 1, state2);
    dfa.set_transition(state2, 2, state1);
    let mut pattern_map = std::collections::BTreeMap::new();
    pattern_map.insert(state1, vec![0]);
    pattern_map.insert(state2, vec![1]);
    dfa.set_pattern_map(&pattern_map).unwrap();
    dfa.minimize();
}

#[test]
fn test_minimize_with_dead_state() {
    let mut dfa: regex_automata::OwnedDFA = regex_automata::OwnedDFA::default();
    let dead_state = dfa.add_empty_state().unwrap();
    let valid_state = dfa.add_empty_state().unwrap();
    dfa.set_transition(valid_state, 255, dead_state);
    dfa.minimize();
}

#[test]
fn test_minimize_without_states_should_panic() {
    let mut dfa: regex_automata::OwnedDFA = regex_automata::OwnedDFA::default();
    // minimize on an empty DFA should panic.
    let result = std::panic::catch_unwind(|| {
        dfa.minimize();
    });
    assert!(result.is_err());
}

