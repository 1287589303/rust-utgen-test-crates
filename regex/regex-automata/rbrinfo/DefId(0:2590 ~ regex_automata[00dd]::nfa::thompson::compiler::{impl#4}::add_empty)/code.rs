fn add_empty(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_empty()
    }