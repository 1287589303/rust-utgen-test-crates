pub fn finish_pattern(
        &mut self,
        start_id: StateID,
    ) -> Result<PatternID, BuildError> {
        let pid = self.current_pattern_id();
        self.start_pattern[pid] = start_id;
        self.pattern_id = None;
        Ok(pid)
    }