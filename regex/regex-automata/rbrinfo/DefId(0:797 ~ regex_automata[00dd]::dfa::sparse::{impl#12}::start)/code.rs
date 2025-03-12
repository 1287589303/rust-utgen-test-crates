fn start(
        &self,
        anchored: Anchored,
        start: Start,
    ) -> Result<StateID, StartError> {
        let start_index = start.as_usize();
        let index = match anchored {
            Anchored::No => {
                if !self.kind.has_unanchored() {
                    return Err(StartError::unsupported_anchored(anchored));
                }
                start_index
            }
            Anchored::Yes => {
                if !self.kind.has_anchored() {
                    return Err(StartError::unsupported_anchored(anchored));
                }
                self.stride + start_index
            }
            Anchored::Pattern(pid) => {
                let len = match self.pattern_len {
                    None => {
                        return Err(StartError::unsupported_anchored(anchored))
                    }
                    Some(len) => len,
                };
                if pid.as_usize() >= len {
                    return Ok(DEAD);
                }
                (2 * self.stride)
                    + (self.stride * pid.as_usize())
                    + start_index
            }
        };
        let start = index * StateID::SIZE;
        // This OK since we're allowed to assume that the start table contains
        // valid StateIDs.
        Ok(wire::read_state_id_unchecked(&self.table()[start..]).0)
    }