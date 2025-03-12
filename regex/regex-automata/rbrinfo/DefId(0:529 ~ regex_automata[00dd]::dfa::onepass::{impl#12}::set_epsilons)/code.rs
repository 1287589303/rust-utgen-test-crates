fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons {
        PatternEpsilons(
            (self.0 & PatternEpsilons::PATTERN_ID_MASK)
                | (u64::from(epsilons.0) & PatternEpsilons::EPSILONS_MASK),
        )
    }