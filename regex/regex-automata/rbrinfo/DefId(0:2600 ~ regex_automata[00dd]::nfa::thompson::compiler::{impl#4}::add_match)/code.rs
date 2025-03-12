fn add_match(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_match()
    }