// Answer 0

#[test]
fn test_initial_partitions_with_no_match_quit_states() {
    struct MockDFA {
        states: Vec<StateID>,
    }

    impl MockDFA {
        fn new() -> Self {
            MockDFA {
                states: vec![StateID(1), StateID(2), StateID(3)],
            }
        }

        fn states(&self) -> &Vec<StateID> {
            &self.states
        }
        
        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _id: StateID) -> bool {
            false
        }

        fn match_len(&self, _id: StateID) -> usize {
            0
        }

        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID {
            PatternID(0)
        }
    }

    let mut dfa = MockDFA::new();
    let result = Minimizer::initial_partitions(&dfa);
}

#[test]
fn test_initial_partitions_with_single_non_matching_state() {
    struct MockDFA {
        states: Vec<StateID>,
    }

    impl MockDFA {
        fn new() -> Self {
            MockDFA {
                states: vec![StateID(1)],
            }
        }

        fn states(&self) -> &Vec<StateID> {
            &self.states
        }
        
        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _id: StateID) -> bool {
            false
        }

        fn match_len(&self, _id: StateID) -> usize {
            0
        }

        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID {
            PatternID(0)
        }
    }

    let mut dfa = MockDFA::new();
    let result = Minimizer::initial_partitions(&dfa);
}

#[test]
fn test_initial_partitions_multiple_states() {
    struct MockDFA {
        states: Vec<StateID>,
    }

    impl MockDFA {
        fn new() -> Self {
            MockDFA {
                states: vec![StateID(1), StateID(4), StateID(5)],
            }
        }

        fn states(&self) -> &Vec<StateID> {
            &self.states
        }
        
        fn is_match_state(&self, _id: StateID) -> bool {
            false
        }

        fn is_quit_state(&self, _id: StateID) -> bool {
            false
        }

        fn match_len(&self, _id: StateID) -> usize {
            0
        }

        fn match_pattern(&self, _id: StateID, _index: usize) -> PatternID {
            PatternID(0)
        }
    }

    let mut dfa = MockDFA::new();
    let result = Minimizer::initial_partitions(&dfa);
}

