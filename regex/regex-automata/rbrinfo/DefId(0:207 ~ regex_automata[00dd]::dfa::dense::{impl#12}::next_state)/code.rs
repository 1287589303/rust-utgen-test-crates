fn next_state(&self, current: StateID, input: u8) -> StateID {
        let input = self.byte_classes().get(input);
        let o = current.as_usize() + usize::from(input);
        self.trans()[o]
    }