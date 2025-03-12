fn add_capture_start(
        &self,
        capture_index: u32,
        name: Option<&str>,
    ) -> Result<StateID, BuildError> {
        let name = name.map(|n| Arc::from(n));
        self.builder.borrow_mut().add_capture_start(
            StateID::ZERO,
            capture_index,
            name,
        )
    }