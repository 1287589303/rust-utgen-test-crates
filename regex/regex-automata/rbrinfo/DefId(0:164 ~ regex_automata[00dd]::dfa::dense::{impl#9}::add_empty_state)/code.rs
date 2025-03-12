pub(crate) fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
        self.tt.add_empty_state()
    }