// Answer 0

#[test]
fn test_new_active_states_zero_capacity() {
    #[derive(Debug)]
    struct MockPikeVM {
        nfa_states: Vec<StateID>,
    }

    impl MockPikeVM {
        fn get_nfa(&self) -> &Self {
            self
        }

        fn states(&self) -> &Vec<StateID> {
            &self.nfa_states
        }
    }

    let mock_nfa_states = vec![0, 1, 2];
    let mock_pike_vm = MockPikeVM {
        nfa_states: mock_nfa_states,
    };
    let active_states = ActiveStates::new(&mock_pike_vm);
}

#[test]
fn test_new_active_states_non_zero_capacity() {
    #[derive(Debug)]
    struct MockPikeVM {
        nfa_states: Vec<StateID>,
    }

    impl MockPikeVM {
        fn get_nfa(&self) -> &Self {
            self
        }

        fn states(&self) -> &Vec<StateID> {
            &self.nfa_states
        }
    }

    let mock_nfa_states = vec![0, 1, 2, 3, 4, 5];
    let mock_pike_vm = MockPikeVM {
        nfa_states: mock_nfa_states,
    };
    let active_states = ActiveStates::new(&mock_pike_vm);
}

#[test]
fn test_new_active_states_empty_nfa() {
    #[derive(Debug)]
    struct MockPikeVM {
        nfa_states: Vec<StateID>,
    }

    impl MockPikeVM {
        fn get_nfa(&self) -> &Self {
            self
        }

        fn states(&self) -> &Vec<StateID> {
            &self.nfa_states
        }
    }

    let mock_nfa_states = vec![];
    let mock_pike_vm = MockPikeVM {
        nfa_states: mock_nfa_states,
    };
    let active_states = ActiveStates::new(&mock_pike_vm);
}

