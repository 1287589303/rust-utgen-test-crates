fn next(&mut self) -> Option<StateID> {
        self.0.next().map(|&id| id)
    }