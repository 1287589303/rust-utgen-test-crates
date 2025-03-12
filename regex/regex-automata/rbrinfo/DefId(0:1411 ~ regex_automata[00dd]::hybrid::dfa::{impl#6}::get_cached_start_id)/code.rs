fn get_cached_start_id(
        &self,
        anchored: Anchored,
        start: Start,
    ) -> Result<LazyStateID, StartError> {
        let start_index = start.as_usize();
        let index = match anchored {
            Anchored::No => start_index,
            Anchored::Yes => Start::len() + start_index,
            Anchored::Pattern(pid) => {
                if !self.dfa.get_config().get_starts_for_each_pattern() {
                    return Err(StartError::unsupported_anchored(anchored));
                }
                if pid.as_usize() >= self.dfa.pattern_len() {
                    return Ok(self.dead_id());
                }
                (2 * Start::len())
                    + (Start::len() * pid.as_usize())
                    + start_index
            }
        };
        Ok(self.cache.starts[index])
    }