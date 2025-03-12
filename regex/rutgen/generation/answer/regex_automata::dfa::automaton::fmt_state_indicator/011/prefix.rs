// Answer 0

#[test]
fn test_fmt_state_indicator_start_non_accelerated() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _: StateID) -> bool {
            true
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
    }

    let mut formatter = core::fmt::Formatter::new();
    let dfa = TestAutomaton;
    let id = StateID::default();
    fmt_state_indicator(&mut formatter, dfa, id).unwrap();
}

#[test]
fn test_fmt_state_indicator_start_accelerated() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _: StateID) -> bool {
            true
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            true
        }

        fn is_match_state(&self, _: StateID) -> bool {
            false
        }
    }

    let mut formatter = core::fmt::Formatter::new();
    let dfa = TestAutomaton;
    let id = StateID::default();
    fmt_state_indicator(&mut formatter, dfa, id).unwrap();
} 

#[test]
fn test_fmt_state_indicator_match_non_accelerated() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _: StateID) -> bool {
            true
        }
    }

    let mut formatter = core::fmt::Formatter::new();
    let dfa = TestAutomaton;
    let id = StateID::default();
    fmt_state_indicator(&mut formatter, dfa, id).unwrap();
}

#[test]
fn test_fmt_state_indicator_match_accelerated() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            true
        }

        fn is_match_state(&self, _: StateID) -> bool {
            true
        }
    }

    let mut formatter = core::fmt::Formatter::new();
    let dfa = TestAutomaton;
    let id = StateID::default();
    fmt_state_indicator(&mut formatter, dfa, id).unwrap();
}

