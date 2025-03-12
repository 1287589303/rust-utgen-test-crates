pub fn add_fail(&mut self) -> Result<StateID, BuildError> {
        self.add(State::Fail)
    }