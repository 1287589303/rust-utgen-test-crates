// Answer 0

#[test]
fn test_next_state_min_state_id_min_input() {
    struct TestAutomaton;
    let automaton = TestAutomaton;
    let state_id = StateID(0);
    let input = 0u8;
    let _ = automaton.next_state(state_id, input);
}

#[test]
fn test_next_state_min_state_id_max_input() {
    struct TestAutomaton;
    let automaton = TestAutomaton;
    let state_id = StateID(0);
    let input = 255u8;
    let _ = automaton.next_state(state_id, input);
}

#[test]
fn test_next_state_mid_state_id_mid_input() {
    struct TestAutomaton;
    let automaton = TestAutomaton;
    let state_id = StateID(128);
    let input = 128u8;
    let _ = automaton.next_state(state_id, input);
}

#[test]
fn test_next_state_max_state_id_min_input() {
    struct TestAutomaton;
    let automaton = TestAutomaton;
    let state_id = StateID(255);
    let input = 0u8;
    let _ = automaton.next_state(state_id, input);
}

#[test]
fn test_next_state_max_state_id_max_input() {
    struct TestAutomaton;
    let automaton = TestAutomaton;
    let state_id = StateID(255);
    let input = 255u8;
    let _ = automaton.next_state(state_id, input);
}

