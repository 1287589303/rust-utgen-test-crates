// Answer 0

#[test]
fn test_match_len_min_state_id() {
    struct DummyAutomaton;
    
    let automaton = DummyAutomaton;
    let state_id = StateID(0); // minimum StateID
    let _ = automaton.match_len(state_id);
}

#[test]
fn test_match_len_edge_case_high_state_id() {
    struct DummyAutomaton;
    
    let automaton = DummyAutomaton;
    let state_id = StateID(u32::MAX as SmallIndex); // maximum StateID
    let _ = automaton.match_len(state_id);
}

#[test]
fn test_match_len_normal_state_id() {
    struct DummyAutomaton;
    
    let automaton = DummyAutomaton;
    let state_id = StateID(1); // normal StateID
    let _ = automaton.match_len(state_id);
}

#[test]
fn test_match_len_multiple_state_ids() {
    struct DummyAutomaton;
    
    let automaton = DummyAutomaton;

    for id in [2, 10, 100, 255].iter() {
        let state_id = StateID(*id); // various normal StateIDs
        let _ = automaton.match_len(state_id);
    }
}

