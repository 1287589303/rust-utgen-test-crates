// Answer 0

#[test]
fn test_fmt_state_indicator_dead_state_not_start() {
    struct TestAutomaton;

    impl Automaton for TestAutomaton {
        fn is_dead_state(&self, id: StateID) -> bool {
            id.0 == 1 // Example dead state ID
        }

        fn is_start_state(&self, id: StateID) -> bool {
            false // Always not a start state
        }

        fn is_quit_state(&self, id: StateID) -> bool {
            false
        }

        fn is_match_state(&self, id: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, id: StateID) -> bool {
            false
        }
    }

    let automaton = TestAutomaton;
    let id = StateID(1); // Example dead state ID
    let mut output = String::new();
    let result = fmt_state_indicator(&mut output, automaton, id);
    assert_eq!(output, "D ");
    assert!(result.is_ok());
}

