fn pattern_id_unchecked(self) -> PatternID {
        let pid = self.0 >> PatternEpsilons::PATTERN_ID_SHIFT;
        PatternID::new_unchecked(pid.as_usize())
    }