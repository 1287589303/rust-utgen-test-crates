pub fn get(&self, i: usize) -> Option<(usize, usize)> {
        let slot = i.checked_mul(2)?;
        let start = self.0.get(slot).copied()??.get();
        let slot = slot.checked_add(1)?;
        let end = self.0.get(slot).copied()??.get();
        Some((start, end))
    }