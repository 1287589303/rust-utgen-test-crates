// Answer 0

#[test]
fn test_fmt_state_indicator_not_special_state() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = DummyAutomaton;
    let state_id = StateID::default();
    let mut formatter = core::fmt::Formatter::new();

    let _ = fmt_state_indicator(&mut formatter, automaton, state_id);
}

#[test]
fn test_fmt_state_indicator_with_custom_id() {
    struct CustomAutomaton;

    impl Automaton for CustomAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_start_state(&self, _: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let automaton = CustomAutomaton;
    let state_id = StateID(SmallIndex::new(1));
    let mut formatter = core::fmt::Formatter::new();

    let _ = fmt_state_indicator(&mut formatter, automaton, state_id);
}

