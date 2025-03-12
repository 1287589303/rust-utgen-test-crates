fn pattern_ids(&self) -> &[PatternID] {
        wire::u32s_to_pattern_ids(self.pattern_ids.as_ref())
    }