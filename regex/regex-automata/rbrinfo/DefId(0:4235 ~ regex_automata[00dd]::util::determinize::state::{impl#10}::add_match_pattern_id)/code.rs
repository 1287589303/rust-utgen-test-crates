fn add_match_pattern_id(&mut self, pid: PatternID) {
        // As a (somewhat small) space saving optimization, in the case where
        // a matching state has exactly one pattern ID, PatternID::ZERO, we do
        // not write either the pattern ID or the number of patterns encoded.
        // Instead, all we do is set the 'is_match' bit on this state. Overall,
        // this saves 8 bytes per match state for the overwhelming majority of
        // match states.
        //
        // In order to know whether pattern IDs need to be explicitly read or
        // not, we use another internal-only bit, 'has_pattern_ids', to
        // indicate whether they have been explicitly written or not.
        if !self.repr().has_pattern_ids() {
            if pid == PatternID::ZERO {
                self.set_is_match();
                return;
            }
            // Make room for 'close_match_pattern_ids' to write the total
            // number of pattern IDs written.
            self.0.extend(core::iter::repeat(0).take(PatternID::SIZE));
            self.set_has_pattern_ids();
            // If this was already a match state, then the only way that's
            // possible when the state doesn't have pattern IDs is if
            // PatternID::ZERO was added by the caller previously. In this
            // case, we are now adding a non-ZERO pattern ID after it, in
            // which case, we want to make sure to represent ZERO explicitly
            // now.
            if self.repr().is_match() {
                write_u32(self.0, 0)
            } else {
                // Otherwise, just make sure the 'is_match' bit is set.
                self.set_is_match();
            }
        }
        write_u32(self.0, pid.as_u32());
    }