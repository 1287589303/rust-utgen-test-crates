fn to_owned(&self) -> MatchStates<alloc::vec::Vec<u32>> {
        MatchStates {
            slices: self.slices.as_ref().to_vec(),
            pattern_ids: self.pattern_ids.as_ref().to_vec(),
            pattern_len: self.pattern_len,
        }
    }