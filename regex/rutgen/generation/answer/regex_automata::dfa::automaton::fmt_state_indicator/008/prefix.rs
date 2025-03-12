// Answer 0

#[test]
fn test_fmt_state_indicator_accelerated_start_state() {
    struct DummyAutomaton;

    impl search::Automaton for DummyAutomaton {
        fn is_dead_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _id: StateID) -> bool {
            true
        }

        fn is_accel_state(&self, _id: StateID) -> bool {
            true
        }

        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID::default();
    let result = core::fmt::Formatter::new();
    fmt_state_indicator(&mut result, automaton, state_id).unwrap();
}

#[test]
fn test_fmt_state_indicator_non_accelerated_start_state() {
    struct DummyAutomaton;

    impl search::Automaton for DummyAutomaton {
        fn is_dead_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _id: StateID) -> bool {
            true
        }

        fn is_accel_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID::default();
    let result = core::fmt::Formatter::new();
    fmt_state_indicator(&mut result, automaton, state_id).unwrap();
}

