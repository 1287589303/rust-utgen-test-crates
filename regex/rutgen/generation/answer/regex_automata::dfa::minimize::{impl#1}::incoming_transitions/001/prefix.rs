// Answer 0

#[test]
fn test_incoming_transitions_states_non_empty() {
    struct TestDFA {
        states: Vec<State<'static>>,
        alphabet_len: usize,
    }

    impl dense::OwnedDFA for TestDFA {
        fn states(&self) -> &[State<'static>] {
            &self.states
        }

        fn alphabet_len(&self) -> usize {
            self.alphabet_len
        }

        fn to_index(&self, state: &StateID) -> usize {
            state.0 .0 as usize
        }
    }

    let state1 = State {
        id: StateID(0),
        stride2: 1,
        transitions: &[StateID(1)],
    };
    let state2 = State {
        id: StateID(1),
        stride2: 1,
        transitions: &[StateID(0)],
    };
    
    let dfa = TestDFA {
        states: vec![state1, state2],
        alphabet_len: 2,
    };

    let incoming = Minimizer::incoming_transitions(&dfa);
}

#[test]
fn test_incoming_transitions_states_empty() {
    struct TestDFA {
        states: Vec<State<'static>>,
        alphabet_len: usize,
    }

    impl dense::OwnedDFA for TestDFA {
        fn states(&self) -> &[State<'static>] {
            &self.states
        }

        fn alphabet_len(&self) -> usize {
            self.alphabet_len
        }

        fn to_index(&self, state: &StateID) -> usize {
            state.0 .0 as usize
        }
    }

    let dfa = TestDFA {
        states: vec![],
        alphabet_len: 1,
    };

    let incoming = Minimizer::incoming_transitions(&dfa);
}

#[test]
fn test_incoming_transitions_with_transitions() {
    struct TestDFA {
        states: Vec<State<'static>>,
        alphabet_len: usize,
    }

    impl dense::OwnedDFA for TestDFA {
        fn states(&self) -> &[State<'static>] {
            &self.states
        }

        fn alphabet_len(&self) -> usize {
            self.alphabet_len
        }

        fn to_index(&self, state: &StateID) -> usize {
            state.0 .0 as usize
        }
    }

    let state1 = State {
        id: StateID(0),
        stride2: 1,
        transitions: &[StateID(1)],
    };
    let state2 = State {
        id: StateID(1),
        stride2: 1,
        transitions: &[StateID(0)],
    };
    
    let dfa = TestDFA {
        states: vec![state1, state2],
        alphabet_len: 2,
    };

    let incoming = Minimizer::incoming_transitions(&dfa);
}

#[test]
fn test_incoming_transitions_without_transitions() {
    struct TestDFA {
        states: Vec<State<'static>>,
        alphabet_len: usize,
    }

    impl dense::OwnedDFA for TestDFA {
        fn states(&self) -> &[State<'static>] {
            &self.states
        }

        fn alphabet_len(&self) -> usize {
            self.alphabet_len
        }

        fn to_index(&self, state: &StateID) -> usize {
            state.0 .0 as usize
        }
    }

    let state = State {
        id: StateID(0),
        stride2: 1,
        transitions: &[],
    };
    
    let dfa = TestDFA {
        states: vec![state],
        alphabet_len: 1,
    };

    let incoming = Minimizer::incoming_transitions(&dfa);
}

