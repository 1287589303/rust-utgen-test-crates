// Answer 0

#[test]
fn test_empty_states_and_starts() {
    struct TestDFA {
        states: Vec<StateID>,
        starts: Vec<(StateID, Anchored, Start)>,
        pattern_len: usize,
        ms: MatchStates<Vec<u32>>,
        state_len: usize,
        flags: Flags,
    }

    impl TestDFA {
        fn states(&self) -> &Vec<StateID> {
            &self.states
        }

        fn starts(&self) -> &Vec<(StateID, Anchored, Start)> {
            &self.starts
        }

        fn pattern_len(&self) -> usize {
            self.pattern_len
        }

        fn state_len(&self) -> usize {
            self.state_len
        }

        fn flags(&self) -> &Flags {
            &self.flags
        }
    }

    let dfa = TestDFA {
        states: vec![],
        starts: vec![],
        pattern_len: 2,
        ms: MatchStates { slices: vec![], pattern_ids: vec![], pattern_len: 0 },
        state_len: 4, // valid size
        flags: Flags { has_empty: false, is_utf8: true, is_always_start_anchored: false },
    };

    let mut buf = String::new();
    let result = dfa.fmt(&mut buf);
}

