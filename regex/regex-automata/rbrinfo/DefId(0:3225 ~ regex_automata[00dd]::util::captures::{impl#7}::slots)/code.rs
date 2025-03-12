pub fn slots(
        &self,
        pid: PatternID,
        group_index: usize,
    ) -> Option<(usize, usize)> {
        // Since 'slot' only even returns valid starting slots, we know that
        // there must also be an end slot and that end slot is always one more
        // than the start slot.
        self.slot(pid, group_index).map(|start| (start, start + 1))
    }