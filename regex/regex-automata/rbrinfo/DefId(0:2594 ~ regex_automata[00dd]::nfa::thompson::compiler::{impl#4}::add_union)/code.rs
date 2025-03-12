fn add_union(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_union(vec![])
    }