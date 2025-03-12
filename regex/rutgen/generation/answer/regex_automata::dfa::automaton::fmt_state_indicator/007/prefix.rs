// Answer 0

#[test]
fn test_fmt_state_indicator_quit_state() {
    struct TestDFA;

    impl Automaton for TestDFA {
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

    let dfa = TestDFA;
    let id = StateID::default();
    let mut output = Vec::new();
    {
        let mut formatter = core::fmt::Formatter::new(&mut output);
        fmt_state_indicator(&mut formatter, dfa, id).unwrap();
    }
    // Output can be checked if necessary, but not part of this requirement
}

