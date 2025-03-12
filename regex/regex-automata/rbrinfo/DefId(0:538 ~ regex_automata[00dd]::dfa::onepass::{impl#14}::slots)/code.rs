fn slots(self) -> Slots {
        Slots((self.0 >> Epsilons::SLOT_SHIFT).low_u32())
    }