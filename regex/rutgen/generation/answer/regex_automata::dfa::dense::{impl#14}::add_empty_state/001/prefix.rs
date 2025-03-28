// Answer 0

#[test]
fn test_add_empty_state_exceeding_state_id_limit() {
    struct TestTransitionTable {
        table: Vec<u32>,
        stride2: usize,
    }
    
    impl TestTransitionTable {
        fn new(max_states: usize) -> Self {
            let stride = 1 << 9; // Max stride corresponding to 512
            let table_capacity = max_states * stride;
            Self { table: vec![0; table_capacity], stride2: 9 }
        }
        
        fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
            let next = self.table.len();
            if next >= MAX_STATES {
                return Err(BuildError::too_many_states());
            }
            self.table.extend(iter::repeat(0).take(1 << self.stride2));
            Ok(StateID::new(next)?)
        }
    }

    const MAX_STATES: usize = 10; // Assume MAX_STATES is 10 for the purpose of the test
    let mut tt = TestTransitionTable::new(MAX_STATES);

    for _ in 0..MAX_STATES {
        let _ = tt.add_empty_state(); // Fill up to the limit
    }
    
    let result = tt.add_empty_state(); // This should exceed the limit
}

#[test]
fn test_add_empty_state_reach_max_capacity() {
    struct TestTransitionTable {
        table: Vec<u32>,
        stride2: usize,
    }

    impl TestTransitionTable {
        fn new(max_states: usize) -> Self {
            let stride = 1 << 9; // Max stride corresponding to 512
            let table_capacity = max_states * stride;
            Self { table: vec![0; table_capacity], stride2: 9 }
        }

        fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
            let next = self.table.len();
            if next >= MAX_STATES {
                return Err(BuildError::too_many_states());
            }
            self.table.extend(iter::repeat(0).take(1 << self.stride2));
            Ok(StateID::new(next)?)
        }
    }

    const MAX_STATES: usize = 6; // Assume MAX_STATES is 6 for the purpose of the test
    let mut tt = TestTransitionTable::new(MAX_STATES);

    for _ in 0..MAX_STATES {
        let _ = tt.add_empty_state(); // Fill up to the limit
    }
    
    let result = tt.add_empty_state(); // This should exceed the capacity
}

