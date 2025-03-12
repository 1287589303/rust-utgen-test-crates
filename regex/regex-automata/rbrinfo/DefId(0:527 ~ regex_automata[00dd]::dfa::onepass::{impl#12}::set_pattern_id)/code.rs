fn set_pattern_id(self, pid: PatternID) -> PatternEpsilons {
        PatternEpsilons(
            (pid.as_u64() << PatternEpsilons::PATTERN_ID_SHIFT)
                | (self.0 & PatternEpsilons::EPSILONS_MASK),
        )
    }