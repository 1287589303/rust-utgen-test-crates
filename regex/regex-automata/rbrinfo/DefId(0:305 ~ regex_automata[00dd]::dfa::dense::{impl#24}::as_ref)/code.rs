fn as_ref(&self) -> MatchStates<&'_ [u32]> {
        MatchStates {
            slices: self.slices.as_ref(),
            pattern_ids: self.pattern_ids.as_ref(),
            pattern_len: self.pattern_len,
        }
    }