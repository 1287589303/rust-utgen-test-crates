// Answer 0

#[test]
fn test_fmt_state_indicator_dead_start_state_write_err() {
    struct TestDFA {
        dead_state: StateID,
        start_state: StateID,
        accel_state: StateID,
    }
    
    impl TestDFA {
        fn new() -> Self {
            Self {
                dead_state: StateID(SmallIndex::new(0)),
                start_state: StateID(SmallIndex::new(1)),
                accel_state: StateID(SmallIndex::new(2)),
            }
        }
    }
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, id: StateID) -> bool {
            id == self.dead_state
        }

        fn is_start_state(&self, id: StateID) -> bool {
            id == self.start_state
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, _: StateID) -> bool {
            false
        }
    }

    let dfa = TestDFA::new();
    let state_id = dfa.dead_state;

    let result = fmt_state_indicator(&mut core::fmt::Formatter::new(), dfa, state_id);
}

#[test]
fn test_fmt_state_indicator_accelerated_dead_start_state_write_err() {
    struct TestDFA {
        dead_state: StateID,
        start_state: StateID,
        accel_state: StateID,
    }
    
    impl TestDFA {
        fn new() -> Self {
            Self {
                dead_state: StateID(SmallIndex::new(0)),
                start_state: StateID(SmallIndex::new(1)),
                accel_state: StateID(SmallIndex::new(2)),
            }
        }
    }
    
    impl Automaton for TestDFA {
        fn is_dead_state(&self, id: StateID) -> bool {
            id == self.dead_state
        }

        fn is_start_state(&self, id: StateID) -> bool {
            id == self.start_state
        }

        fn is_quit_state(&self, _: StateID) -> bool {
            false
        }

        fn is_match_state(&self, _: StateID) -> bool {
            false
        }

        fn is_accel_state(&self, id: StateID) -> bool {
            id == self.accel_state
        }
    }

    let dfa = TestDFA::new();
    let state_id = dfa.dead_state;

    let result = fmt_state_indicator(&mut core::fmt::Formatter::new(), dfa, state_id);
}

