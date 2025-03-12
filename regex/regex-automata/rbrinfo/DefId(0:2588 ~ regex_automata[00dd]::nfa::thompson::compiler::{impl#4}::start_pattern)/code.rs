fn start_pattern(&self) -> Result<PatternID, BuildError> {
        self.builder.borrow_mut().start_pattern()
    }