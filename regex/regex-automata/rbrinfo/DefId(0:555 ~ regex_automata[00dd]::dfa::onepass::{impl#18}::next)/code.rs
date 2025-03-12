fn next(&mut self) -> Option<usize> {
        // Number of zeroes here is always <= u8::MAX, and so fits in a usize.
        let slot = self.slots.0.trailing_zeros().as_usize();
        if slot >= Slots::LIMIT {
            return None;
        }
        self.slots = self.slots.remove(slot);
        Some(slot)
    }