pub fn explicit_slot_len(&self) -> usize {
        self.slot_len().saturating_sub(self.implicit_slot_len())
    }