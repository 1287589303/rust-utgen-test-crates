// Answer 0

#[test]
fn test_fmt_state_indicator_start_accel() {
    struct TestDFA;
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { true }
        fn is_match_state(&self, _: StateID) -> bool { false }
    }

    let dfa = TestDFA;
    let id = StateID(Default::default());
    let mut output = String::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        fmt_state_indicator(&mut formatter, dfa, id).unwrap();
    }
}

#[test]
fn test_fmt_state_indicator_start_non_accel() {
    struct TestDFA;
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { true }
        fn is_accel_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { false }
    }

    let dfa = TestDFA;
    let id = StateID(Default::default());
    let mut output = String::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        fmt_state_indicator(&mut formatter, dfa, id).unwrap();
    }
}

#[test]
fn test_fmt_state_indicator_non_start() {
    struct TestDFA;
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { false }
        fn is_accel_state(&self, _: StateID) -> bool { true }
        fn is_match_state(&self, _: StateID) -> bool { false }
    }

    let dfa = TestDFA;
    let id = StateID(Default::default());
    let mut output = String::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        fmt_state_indicator(&mut formatter, dfa, id).unwrap();
    }
}

