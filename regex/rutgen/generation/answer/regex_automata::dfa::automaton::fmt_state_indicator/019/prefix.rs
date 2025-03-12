// Answer 0

#[test]
fn test_fmt_state_indicator_non_special_state() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool { false }
        fn is_quit_state(&self, _: StateID) -> bool { false }
        fn is_start_state(&self, _: StateID) -> bool { false }
        fn is_match_state(&self, _: StateID) -> bool { false }
        fn is_accel_state(&self, _: StateID) -> bool { false }
    }

    let automaton = TestAutomaton;
    let id = StateID::default(); // Assuming a default StateID is not a special state
    let mut formatter = core::fmt::Formatter::new();

    let _ = fmt_state_indicator(&mut formatter, automaton, id);
}

