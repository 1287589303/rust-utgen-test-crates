fn add_sparse(
        &self,
        ranges: Vec<Transition>,
    ) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_sparse(ranges)
    }