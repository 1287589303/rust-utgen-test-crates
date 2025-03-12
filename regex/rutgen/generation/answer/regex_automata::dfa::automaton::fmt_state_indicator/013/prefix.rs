// Answer 0

#[test]
fn test_fmt_state_indicator_accel_match() {
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
            true
        }
    }

    let dfa = TestDFA;
    let id = StateID(SmallIndex::new(0));
    let mut output = String::new();
    let _ = fmt_state_indicator(&mut output, dfa, id);
}

#[test]
fn test_fmt_state_indicator_non_accel_match() {
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
    let id = StateID(SmallIndex::new(1));
    let mut output = String::new();
    let _ = fmt_state_indicator(&mut output, dfa, id);
}

