fn add_fail(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_fail()
    }