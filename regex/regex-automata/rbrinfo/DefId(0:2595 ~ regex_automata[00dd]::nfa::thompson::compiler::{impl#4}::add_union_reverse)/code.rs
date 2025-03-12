fn add_union_reverse(&self) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_union_reverse(vec![])
    }