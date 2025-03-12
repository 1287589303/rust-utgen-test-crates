fn set_slots(self, slots: Slots) -> Epsilons {
        Epsilons(
            (u64::from(slots.0) << Epsilons::SLOT_SHIFT)
                | (self.0 & Epsilons::LOOK_MASK),
        )
    }