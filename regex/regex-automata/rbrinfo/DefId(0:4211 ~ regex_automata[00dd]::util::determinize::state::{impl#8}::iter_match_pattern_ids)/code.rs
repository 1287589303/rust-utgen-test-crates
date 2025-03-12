fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {
        if !self.is_match() {
            return;
        }
        // As an optimization for a very common case, when this is a match
        // state for an NFA with only one pattern, we don't actually write the
        // pattern ID to the state representation. Instead, we know it must
        // be there since it is the only possible choice.
        if !self.has_pattern_ids() {
            f(PatternID::ZERO);
            return;
        }
        let mut pids = &self.0[13..self.pattern_offset_end()];
        while !pids.is_empty() {
            let pid = wire::read_u32(pids);
            pids = &pids[PatternID::SIZE..];
            // This is OK since we only ever serialize valid PatternIDs to
            // states. And since pattern IDs can never exceed a usize, the
            // unwrap is OK.
            f(PatternID::new_unchecked(usize::try_from(pid).unwrap()));
        }
    }