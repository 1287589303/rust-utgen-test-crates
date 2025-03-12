fn looks(self) -> LookSet {
        LookSet { bits: (self.0 & Epsilons::LOOK_MASK).low_u32() }
    }