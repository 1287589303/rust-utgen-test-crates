// Answer 0

#[test]
fn test_shuffle_with_valid_conditions() {
    use alloc::collections::BTreeMap;

    struct TestDFA {
        special: Special,
        state_len: usize,
        starts: Vec<(StateID, usize, usize)>,
        transitions: BTreeMap<StateID, Vec<PatternID>>,
    }

    impl TestDFA {
        fn new(state_len: usize, starts: Vec<(StateID, usize, usize)>, matches: BTreeMap<StateID, Vec<PatternID>>) -> Self {
            let mut special = Special::new();
            special.quit_id = StateID(1);

            TestDFA { 
                special, 
                state_len, 
                starts, 
                transitions: matches 
            }
        }

        fn to_state_id(&self, id: usize) -> StateID {
            StateID(id as u32)
        }

        fn state_len(&self) -> usize {
            self.state_len
        }

        fn starts(&self) -> &[(StateID, usize, usize)] {
            &self.starts
        }

        fn shuffle(&mut self) -> Result<(), BuildError> {
            shuffle(self, self.transitions.clone())
        }
    }

    let starts = vec![
        (StateID(2), 0, 0), // valid start states
        (StateID(3), 0, 0)
    ];
    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::from([
        (StateID(4), vec![PatternID(0)]), // valid matches
        (StateID(5), vec![PatternID(1)]),
    ]);

    let mut dfa = TestDFA::new(3, starts, matches);
    dfa.shuffle().unwrap();
}

#[test]
#[should_panic]
fn test_shuffle_with_invalid_start_id() {
    use alloc::collections::BTreeMap;

    struct TestDFA {
        special: Special,
        state_len: usize,
        starts: Vec<(StateID, usize, usize)>,
        transitions: BTreeMap<StateID, Vec<PatternID>>,
    }

    impl TestDFA {
        fn new(state_len: usize, starts: Vec<(StateID, usize, usize)>, matches: BTreeMap<StateID, Vec<PatternID>>) -> Self {
            let mut special = Special::new();
            special.quit_id = StateID(1);

            TestDFA { 
                special, 
                state_len, 
                starts, 
                transitions: matches 
            }
        }

        fn to_state_id(&self, id: usize) -> StateID {
            StateID(id as u32)
        }

        fn state_len(&self) -> usize {
            self.state_len
        }

        fn starts(&self) -> &[(StateID, usize, usize)] {
            &self.starts
        }

        fn shuffle(&mut self) -> Result<(), BuildError> {
            shuffle(self, self.transitions.clone())
        }
    }

    let starts = vec![
        (StateID(0), 0, 0) // DEAD state (valid start_id cannot be DEAD)
    ];
    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::from([]);

    let mut dfa = TestDFA::new(3, starts, matches);
    dfa.shuffle().unwrap(); // This should panic since starts can't be DEAD
}

