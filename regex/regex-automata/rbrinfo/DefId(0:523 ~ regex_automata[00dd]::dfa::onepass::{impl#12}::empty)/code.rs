fn empty() -> PatternEpsilons {
        PatternEpsilons(
            PatternEpsilons::PATTERN_ID_NONE
                << PatternEpsilons::PATTERN_ID_SHIFT,
        )
    }