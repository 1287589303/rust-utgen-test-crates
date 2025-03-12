pub fn len(&self) -> usize {
        // self.0.group_len() returns 0 if the underlying captures doesn't
        // represent a match, but the behavior guaranteed for this method is
        // that the length doesn't change based on a match or not.
        self.0.group_info().group_len(PatternID::ZERO)
    }