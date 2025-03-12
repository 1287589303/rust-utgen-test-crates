pub fn add_match(&mut self) -> Result<StateID, BuildError> {
        let pattern_id = self.current_pattern_id();
        let sid = self.add(State::Match { pattern_id })?;
        Ok(sid)
    }