// Answer 0

#[test]
fn test_fmt_state_indicator_match_accel_state() {
    struct TestDFA;
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { true }
    }

    let mut output = String::new();
    let dfa = TestDFA;
    let id = StateID(Default::default());
    
    let _ = fmt_state_indicator(&mut output, dfa, id);
}

#[test]
fn test_fmt_state_indicator_match_accel_state_with_custom_id() {
    struct TestDFA;
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { true }
    }

    let mut output = String::new();
    let dfa = TestDFA;
    let id = StateID(Default::default());
    
    let _ = fmt_state_indicator(&mut output, dfa, id);
}

