pub fn add_empty(&mut self) -> Result<StateID, BuildError> {
        self.add(State::Empty { next: StateID::ZERO })
    }