// Answer 0

#[test]
fn test_remap_with_no_swaps() {
    struct MockRemappable {
        states: Vec<StateID>,
    }

    impl MockRemappable {
        fn new(states: Vec<StateID>) -> Self {
            MockRemappable { states }
        }

        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn remap<F>(&mut self, f: F)
        where
            F: FnMut(StateID),
        {
            for &state in &self.states {
                f(state);
            }
        }
    }

    let states = vec![StateID(0), StateID(1), StateID(2)];
    let mut remappable = MockRemappable::new(states);
    let mut remapper = Remapper::new(&remappable);

    remapper.remap(&mut remappable);
}

#[test]
fn test_remap_with_single_swap() {
    struct MockRemappable {
        states: Vec<StateID>,
    }

    impl MockRemappable {
        fn new(states: Vec<StateID>) -> Self {
            MockRemappable { states }
        }

        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn remap<F>(&mut self, f: F)
        where
            F: FnMut(StateID),
        {
            for &state in &self.states {
                f(state);
            }
        }
    }

    let states = vec![StateID(0), StateID(1), StateID(2)];
    let mut remappable = MockRemappable::new(states);
    let mut remapper = Remapper::new(&remappable);
    remapper.swap(&mut remappable, StateID(0), StateID(1));
    
    remapper.remap(&mut remappable);
}

#[test]
#[should_panic]
fn test_remap_with_index_out_of_bounds() {
    struct MockRemappable {
        states: Vec<StateID>,
    }

    impl MockRemappable {
        fn new(states: Vec<StateID>) -> Self {
            MockRemappable { states }
        }

        fn state_len(&self) -> usize {
            self.states.len()
        }

        fn remap<F>(&mut self, f: F)
        where
            F: FnMut(StateID),
        {
            for &state in &self.states {
                f(state);
            }
        }
    }

    let states = vec![StateID(0), StateID(1), StateID(2)];
    let mut remappable = MockRemappable::new(states);
    let mut remapper = Remapper::new(&remappable);
    
    // Let's set the state length to 2 so that index 2 is out of bounds
    let invalid_index = 2;
    remapper.map.push(StateID(invalid_index));
    
    remapper.remap(&mut remappable);
}

