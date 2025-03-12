// Answer 0

#[test]
fn test_next_state_unchecked_valid_stateid_byte_min() {
    struct TestAutomaton;
    
    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { StateID(SmallIndex::new(0)) }
    }
    
    let automaton = TestAutomaton;
    let current = StateID(SmallIndex::new(0)); // valid StateID
    let input = 0; // minimum byte value
    unsafe {
        let _ = automaton.next_state_unchecked(current, input);
    }
}

#[test]
fn test_next_state_unchecked_valid_stateid_byte_max() {
    struct TestAutomaton;
    
    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { StateID(SmallIndex::new(1)) }
    }
    
    let automaton = TestAutomaton;
    let current = StateID(SmallIndex::new(1)); // valid StateID
    let input = 255; // maximum byte value
    unsafe {
        let _ = automaton.next_state_unchecked(current, input);
    }
}

#[test]
fn test_next_state_unchecked_valid_stateid_byte_mid() {
    struct TestAutomaton;
    
    unsafe impl Automaton for TestAutomaton {
        fn next_state(&self, _: StateID, _: u8) -> StateID { StateID(SmallIndex::new(2)) }
    }
    
    let automaton = TestAutomaton;
    let current = StateID(SmallIndex::new(2)); // valid StateID
    let input = 128; // middle byte value
    unsafe {
        let _ = automaton.next_state_unchecked(current, input);
    }
}

