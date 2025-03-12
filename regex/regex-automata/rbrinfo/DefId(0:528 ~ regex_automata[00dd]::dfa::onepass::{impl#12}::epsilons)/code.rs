fn epsilons(self) -> Epsilons {
        Epsilons(self.0 & PatternEpsilons::EPSILONS_MASK)
    }