fn cache_start_group(
        &mut self,
        anchored: Anchored,
        start: Start,
    ) -> Result<LazyStateID, StartError> {
        let nfa_start_id = match anchored {
            Anchored::No => self.dfa.get_nfa().start_unanchored(),
            Anchored::Yes => self.dfa.get_nfa().start_anchored(),
            Anchored::Pattern(pid) => {
                if !self.dfa.get_config().get_starts_for_each_pattern() {
                    return Err(StartError::unsupported_anchored(anchored));
                }
                match self.dfa.get_nfa().start_pattern(pid) {
                    None => return Ok(self.as_ref().dead_id()),
                    Some(sid) => sid,
                }
            }
        };

        let id = self
            .cache_start_one(nfa_start_id, start)
            .map_err(StartError::cache)?;
        self.set_start_state(anchored, start, id);
        Ok(id)
    }