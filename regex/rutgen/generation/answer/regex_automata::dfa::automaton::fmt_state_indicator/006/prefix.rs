// Answer 0

#[test]
fn test_fmt_state_indicator_quit_state() {
    struct DummyAutomaton;

    impl Automaton for DummyAutomaton {
        fn is_dead_state(&self, _id: StateID) -> bool {
            false
        }
        
        fn is_quit_state(&self, _id: StateID) -> bool {
            true
        }

        fn is_start_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _id: StateID) -> bool {
            false
        }
    }

    let mut buffer = core::fmt::Formatter::new();
    let automaton = DummyAutomaton;
    let state_id = StateID(Default::default());
    
    fmt_state_indicator(&mut buffer, automaton, state_id).unwrap();
}

#[test]
fn test_fmt_state_indicator_not_dead_not_quit() {
    struct AnotherDummyAutomaton;

    impl Automaton for AnotherDummyAutomaton {
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
            false
        }
    }

    let mut buffer = core::fmt::Formatter::new();
    let automaton = AnotherDummyAutomaton;
    let state_id = StateID(Default::default());
    
    fmt_state_indicator(&mut buffer, automaton, state_id).unwrap();
}

