fn root(&mut self) -> usize {
        if !self.states.is_empty() {
            0
        } else {
            self.create_state()
        }
    }