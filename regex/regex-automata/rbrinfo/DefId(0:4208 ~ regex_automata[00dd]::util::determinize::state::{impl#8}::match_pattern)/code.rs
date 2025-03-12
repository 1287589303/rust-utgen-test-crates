fn match_pattern(&self, index: usize) -> PatternID {
        if !self.has_pattern_ids() {
            PatternID::ZERO
        } else {
            let offset = 13 + index * PatternID::SIZE;
            // This is OK since we only ever serialize valid PatternIDs to
            // states.
            wire::read_pattern_id_unchecked(&self.0[offset..]).0
        }
    }