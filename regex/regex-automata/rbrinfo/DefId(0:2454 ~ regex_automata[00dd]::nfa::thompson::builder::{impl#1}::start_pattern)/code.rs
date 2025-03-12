pub fn start_pattern(&mut self) -> Result<PatternID, BuildError> {
        assert!(self.pattern_id.is_none(), "must call 'finish_pattern' first");

        let proposed = self.start_pattern.len();
        let pid = PatternID::new(proposed)
            .map_err(|_| BuildError::too_many_patterns(proposed))?;
        self.pattern_id = Some(pid);
        // This gets filled in when 'finish_pattern' is called.
        self.start_pattern.push(StateID::ZERO);
        Ok(pid)
    }