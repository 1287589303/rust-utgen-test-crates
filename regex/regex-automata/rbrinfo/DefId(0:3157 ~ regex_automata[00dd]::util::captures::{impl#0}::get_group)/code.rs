pub fn get_group(&self, index: usize) -> Option<Span> {
        let pid = self.pattern()?;
        // There's a little bit of work needed to map captures to slots in the
        // fully general case. But in the overwhelming common case of a single
        // pattern, we can just do some simple arithmetic.
        let (slot_start, slot_end) = if self.group_info().pattern_len() == 1 {
            (index.checked_mul(2)?, index.checked_mul(2)?.checked_add(1)?)
        } else {
            self.group_info().slots(pid, index)?
        };
        let start = self.slots.get(slot_start).copied()??;
        let end = self.slots.get(slot_end).copied()??;
        Some(Span { start: start.get(), end: end.get() })
    }