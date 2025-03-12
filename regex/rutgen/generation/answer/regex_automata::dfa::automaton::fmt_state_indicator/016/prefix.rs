// Answer 0

#[test]
fn test_fmt_state_indicator_accelerated_state() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, _id: StateID) -> bool {
            false
        }
        fn is_quit_state(&self, _id: StateID) -> bool {
            false
        }
        fn is_start_state(&self, _id: StateID) -> bool {
            false
        }
        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }
        fn is_accel_state(&self, _id: StateID) -> bool {
            true
        }
    }

    let dfa = TestAutomaton;
    let id = StateID::default();
    let mut output = String::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        fmt_state_indicator(&mut formatter, dfa, id).unwrap();
    }
    // The output of the testing scenario is in output, which would be "A ".
}

