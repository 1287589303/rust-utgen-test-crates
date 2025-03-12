fn epsilons(&self) -> Epsilons {
        Epsilons(self.0 & Transition::INFO_MASK)
    }