fn start_pattern(&self, pid: PatternID) -> Result<StateID, MatchError> {
        if !self.config.get_starts_for_each_pattern() {
            return Err(MatchError::unsupported_anchored(Anchored::Pattern(
                pid,
            )));
        }
        // 'starts' always has non-zero length. The first entry is always the
        // anchored starting state for all patterns, and the following entries
        // are optional and correspond to the anchored starting states for
        // patterns at pid+1. Thus, starts.len()-1 corresponds to the total
        // number of patterns that one can explicitly search for. (And it may
        // be zero.)
        Ok(self.starts.get(pid.one_more()).copied().unwrap_or(DEAD))
    }