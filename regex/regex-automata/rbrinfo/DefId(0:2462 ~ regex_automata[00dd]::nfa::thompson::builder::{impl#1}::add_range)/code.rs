pub fn add_range(
        &mut self,
        trans: Transition,
    ) -> Result<StateID, BuildError> {
        self.add(State::ByteRange { trans })
    }