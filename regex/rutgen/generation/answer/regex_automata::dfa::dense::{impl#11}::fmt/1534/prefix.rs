// Answer 0

#[test]
fn test_fmt_empty_dfa() {
    use core::fmt::Formatter;

    struct TestDFA {
        states: Vec<StateID>,
        starts: Vec<(StateID, Anchored, Start)>,
        pattern_len: usize,
        state_len: usize,
    }

    impl TestDFA {
        fn states(&self) -> impl Iterator<Item = &StateID> {
            self.states.iter()
        }

        fn starts(&self) -> impl Iterator<Item = &(StateID, Anchored, Start)> {
            self.starts.iter()
        }

        fn pattern_len(&self) -> usize {
            self.pattern_len
        }

        fn state_len(&self) -> usize {
            self.state_len
        }
    }

    let mut formatter = Formatter::new();
    let dfa = TestDFA {
        states: vec![],
        starts: vec![],
        pattern_len: 1,
        state_len: usize::MAX, // trigger a failure condition
    };

    let _ = dfa.fmt(&mut formatter);
}

