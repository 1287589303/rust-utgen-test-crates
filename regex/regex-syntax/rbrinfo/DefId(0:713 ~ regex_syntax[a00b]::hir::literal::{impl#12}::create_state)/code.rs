fn create_state(&mut self) -> usize {
        let id = self.states.len();
        self.states.push(State::default());
        self.matches.push(None);
        id
    }