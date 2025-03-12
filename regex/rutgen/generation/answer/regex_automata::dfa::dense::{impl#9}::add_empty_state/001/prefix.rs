// Answer 0

#[test]
fn test_add_empty_state_success() {
    struct TestDFA {
        tt: DummyTransitionTable,
    }

    struct DummyTransitionTable {
        states: Vec<StateID>,
    }

    impl DummyTransitionTable {
        fn new() -> Self {
            DummyTransitionTable { states: Vec::new() }
        }

        fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
            let new_state_id = StateID::default();
            // Simulate adding an empty state
            if self.states.len() < StateID::LIMIT {
                self.states.push(new_state_id);
                Ok(new_state_id)
            } else {
                Err(BuildError { kind: BuildErrorKind::LimitExceeded })
            }
        }
    }

    let mut dfa = TestDFA { tt: DummyTransitionTable::new() };
    let result = dfa.add_empty_state();
}

#[test]
fn test_add_empty_state_limit_exceeded() {
    struct TestDFA {
        tt: DummyTransitionTable,
    }

    struct DummyTransitionTable {
        states: Vec<StateID>,
    }

    impl DummyTransitionTable {
        fn new() -> Self {
            DummyTransitionTable { states: vec![StateID::default(); StateID::LIMIT] }
        }

        fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
            let new_state_id = StateID::default();
            if self.states.len() < StateID::LIMIT {
                self.states.push(new_state_id);
                Ok(new_state_id)
            } else {
                Err(BuildError { kind: BuildErrorKind::LimitExceeded })
            }
        }
    }

    let mut dfa = TestDFA { tt: DummyTransitionTable::new() };
    let result = dfa.add_empty_state();
}

#[test]
fn test_add_empty_state_unique_id() {
    struct TestDFA {
        tt: DummyTransitionTable,
    }

    struct DummyTransitionTable {
        states: Vec<StateID>,
    }

    impl DummyTransitionTable {
        fn new() -> Self {
            DummyTransitionTable { states: Vec::new() }
        }

        fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
            let new_state_id = StateID::default();
            if !self.states.contains(&new_state_id) && self.states.len() < StateID::LIMIT {
                self.states.push(new_state_id);
                Ok(new_state_id)
            } else {
                Err(BuildError { kind: BuildErrorKind::DuplicateState })
            }
        }
    }

    let mut dfa = TestDFA { tt: DummyTransitionTable::new() };
    let id1 = dfa.add_empty_state().unwrap();
    let id2 = dfa.add_empty_state().unwrap();
}

