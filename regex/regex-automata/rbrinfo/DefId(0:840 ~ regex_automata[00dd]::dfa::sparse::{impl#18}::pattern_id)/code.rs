fn pattern_id(&self, match_index: usize) -> PatternID {
        let start = match_index * PatternID::SIZE;
        wire::read_pattern_id_unchecked(&self.pattern_ids[start..]).0
    }