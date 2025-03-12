// Answer 0

#[test]
fn test_from_dense_case_1() {
    let dense_dfa: dense::DFA<Vec<u32>> = dense::DFA::always_match().unwrap();
    let result = DFA::from_dense(&dense_dfa);
}

#[test]
fn test_from_dense_case_2() {
    let dense_dfa: dense::DFA<Vec<u32>> = dense::DFA::never_match().unwrap();
    let result = DFA::from_dense(&dense_dfa);
}

#[test]
fn test_from_dense_case_3() {
    struct DummyDenseDFA {
        state_len: usize,
        match_states: Vec<bool>,
    }

    impl DummyDenseDFA {
        fn states(&self) -> impl Iterator<Item = StateID> {
            (0..self.state_len).map(StateID)
        }
        
        fn is_match_state(&self, id: StateID) -> bool {
            self.match_states[id.0 as usize]
        }
        
        fn to_index(&self, id: StateID) -> usize {
            id.0 as usize
        }
        
        fn pattern_len(&self) -> usize {
            1
        }
    }

    let dense_dfa = DummyDenseDFA {
        state_len: 2,
        match_states: vec![false, false],
    };
    
    let result = DFA::from_dense(&dense_dfa);
} 

#[test]
fn test_from_dense_case_4() {
    struct DummyDenseDFA {
        state_len: usize,
        match_states: Vec<bool>,
    }

    impl DummyDenseDFA {
        fn states(&self) -> impl Iterator<Item = StateID> {
            (0..self.state_len).map(StateID)
        }
        
        fn is_match_state(&self, id: StateID) -> bool {
            self.match_states[id.0 as usize]
        }

        fn to_index(&self, id: StateID) -> usize {
            id.0 as usize
        }

        fn pattern_len(&self) -> usize {
            1
        }
    }

    let dense_dfa = DummyDenseDFA {
        state_len: 3,
        match_states: vec![false, false, false],
    };

    let result = DFA::from_dense(&dense_dfa);
}

#[test]
fn test_from_dense_case_5() {
    struct DummyDenseDFA {
        state_len: usize,
        match_states: Vec<bool>,
    }

    impl DummyDenseDFA {
        fn states(&self) -> impl Iterator<Item = StateID> {
            (0..self.state_len).map(StateID)
        }

        fn is_match_state(&self, id: StateID) -> bool {
            self.match_states[id.0 as usize]
        }

        fn to_index(&self, id: StateID) -> usize {
            id.0 as usize
        }

        fn pattern_len(&self) -> usize {
            2
        }
    }

    let dense_dfa = DummyDenseDFA {
        state_len: 1,
        match_states: vec![false],
    };

    let result = DFA::from_dense(&dense_dfa);
}

