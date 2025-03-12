fn add_range(&self, start: u8, end: u8) -> Result<StateID, BuildError> {
        self.builder.borrow_mut().add_range(Transition {
            start,
            end,
            next: StateID::ZERO,
        })
    }