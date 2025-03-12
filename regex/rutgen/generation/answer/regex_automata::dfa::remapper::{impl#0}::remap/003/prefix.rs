// Answer 0

#[test]
fn test_remap_with_swapped_states() {
    struct TestRemappable {
        states: Vec<StateID>,
    }

    impl TestRemappable {
        fn new(states: Vec<StateID>) -> Self {
            Self { states }
        }
    }

    impl Remappable for TestRemappable {
        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn remap<F: FnMut(StateID)>(&mut self, mut f: F) {
            for id in &self.states {
                f(*id);
            }
        }
    }

    let mut remapper = Remapper {
        map: vec![StateID(1), StateID(2), StateID(3)], // Initial mapping
        idxmap: IndexMapper { stride2: 1 },
    };

    let states = vec![StateID(10), StateID(20), StateID(30)];
    let mut remappable = TestRemappable::new(states);
    
    remapper.remap(&mut remappable);
}

#[test]
fn test_remap_with_looping_swaps() {
    struct TestRemappable {
        states: Vec<StateID>,
    }

    impl TestRemappable {
        fn new(states: Vec<StateID>) -> Self {
            Self { states }
        }
    }

    impl Remappable for TestRemappable {
        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn remap<F: FnMut(StateID)>(&mut self, mut f: F) {
            for id in &self.states {
                f(*id);
            }
        }
    }

    let mut remapper = Remapper {
        map: vec![StateID(5), StateID(6), StateID(7)], // Initial mapping
        idxmap: IndexMapper { stride2: 1 },
    };

    let states = vec![StateID(15), StateID(25), StateID(35)];
    let mut remappable = TestRemappable::new(states);
    
    remapper.remap(&mut remappable);
}

#[test]
fn test_remap_empty_state_length() {
    struct TestRemappable {
        states: Vec<StateID>,
    }

    impl TestRemappable {
        fn new(states: Vec<StateID>) -> Self {
            Self { states }
        }
    }

    impl Remappable for TestRemappable {
        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn remap<F: FnMut(StateID)>(&mut self, _: F) {
            // No states to remap
        }
    }

    let mut remapper = Remapper {
        map: vec![], // No mapping
        idxmap: IndexMapper { stride2: 1 },
    };

    let states = vec![]; // Empty state list
    let mut remappable = TestRemappable::new(states);
    
    remapper.remap(&mut remappable);
}

