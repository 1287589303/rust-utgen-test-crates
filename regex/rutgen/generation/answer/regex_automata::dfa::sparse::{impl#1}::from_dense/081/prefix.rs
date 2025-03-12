// Answer 0

#[test]
fn test_from_dense_empty_transition_length() {
    let dfa = {
        struct DummyDFA {
            states: Vec<StateID>,
            pattern_len: usize,
            has_empty: bool,
        }
        
        impl DummyDFA {
            fn state_len(&self) -> usize {
                self.states.len()
            }

            fn states(&self) -> &Vec<StateID> {
                &self.states
            }
            
            fn pattern_len(&self) -> usize {
                self.pattern_len
            }

            fn has_empty(&self) -> bool {
                self.has_empty
            }

            fn to_index(&self, id: StateID) -> usize {
                id.0 as usize
            }
        }

        let states = vec![StateID(0)];
        DummyDFA {
            states,
            pattern_len: 258,
            has_empty: true,
        }
    };

    let result = DFA::from_dense(&dfa);
}

#[test]
fn test_from_dense_high_pattern_length() {
    let dfa = {
        struct DummyDFA {
            states: Vec<StateID>,
            pattern_len: usize,
            has_empty: bool,
        }
        
        impl DummyDFA {
            fn state_len(&self) -> usize {
                self.states.len()
            }

            fn states(&self) -> &Vec<StateID> {
                &self.states
            }
            
            fn pattern_len(&self) -> usize {
                self.pattern_len
            }

            fn has_empty(&self) -> bool {
                self.has_empty
            }

            fn to_index(&self, id: StateID) -> usize {
                id.0 as usize
            }
        }

        let states = vec![StateID(0)];
        DummyDFA {
            states,
            pattern_len: 300,
            has_empty: true,
        }
    };

    let result = DFA::from_dense(&dfa);
}

