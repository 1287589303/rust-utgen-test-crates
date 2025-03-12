pub fn captures_len(&self) -> usize {
        self.meta.group_info().group_len(PatternID::ZERO)
    }