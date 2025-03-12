fn finish_pattern(
        &self,
        start_id: StateID,
    ) -> Result<PatternID, BuildError> {
        self.builder.borrow_mut().finish_pattern(start_id)
    }