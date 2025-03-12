fn add_capture_end(
        &self,
        capture_index: u32,
    ) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_capture_end(StateID::ZERO, capture_index)
    }