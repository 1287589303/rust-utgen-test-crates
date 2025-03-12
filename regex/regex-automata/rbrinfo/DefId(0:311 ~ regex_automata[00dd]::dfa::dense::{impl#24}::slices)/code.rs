fn slices(&self) -> &[PatternID] {
        wire::u32s_to_pattern_ids(self.slices.as_ref())
    }