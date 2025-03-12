// Answer 0

#[test]
fn test_fmt_state_indicator_match_non_accel() {
    struct TestAutomaton {
        dead_state: bool,
        quit_state: bool,
        start_state: bool,
        match_state: bool,
        accel_state: bool,
    }

    impl TestAutomaton {
        fn new() -> Self {
            TestAutomaton {
                dead_state: false,
                quit_state: false,
                start_state: false,
                match_state: true,
                accel_state: false,
            }
        }
    }

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _: StateID) -> bool {
            self.dead_state
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            self.quit_state
        }

        fn is_start_state(&self, _: StateID) -> bool {
            self.start_state
        }

        fn is_match_state(&self, _: StateID) -> bool {
            self.match_state
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            self.accel_state
        }
    }

    let automaton = TestAutomaton::new();
    let state_id = StateID::default();
    let mut output = core::fmt::Formatter::new();

    fmt_state_indicator(&mut output, automaton, state_id).unwrap();
}

