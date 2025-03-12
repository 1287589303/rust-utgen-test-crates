fn set_looks(self, look_set: LookSet) -> Epsilons {
        Epsilons(
            (self.0 & Epsilons::SLOT_MASK)
                | (u64::from(look_set.bits) & Epsilons::LOOK_MASK),
        )
    }