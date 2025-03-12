fn add_empty(&self) -> Result<StateID, Error> {
        self.add(State::Goto { target: 0, look: None })
    }