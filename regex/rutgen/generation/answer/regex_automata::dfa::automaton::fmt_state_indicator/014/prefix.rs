// Answer 0

#[test]
fn test_fmt_state_indicator_match_state() {
    struct TestDFA;

    impl Automaton for TestDFA {
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
            true
        }

        fn is_accel_state(&self, _id: StateID) -> bool {
            false
        }
    }

    let dfa = TestDFA;
    let state_id = StateID(SmallIndex::default());
    let mut buffer = core::fmt::Formatter::new();
    
    let _ = fmt_state_indicator(&mut buffer, dfa, state_id);
}

#[test]
fn test_fmt_state_indicator_error_case() {
    struct TestDFA;

    impl Automaton for TestDFA {
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
            true
        }

        fn is_accel_state(&self, _id: StateID) -> bool {
            false
        }
    }

    let dfa = TestDFA;
    let state_id = StateID(SmallIndex::default());
    let mut buffer = core::fmt::Formatter::new();

    let _ = fmt_state_indicator(&mut buffer, dfa, state_id);
}

