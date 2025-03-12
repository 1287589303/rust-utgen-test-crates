pub fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize> {
        if group_index >= self.group_len(pid) {
            return None;
        }
        // At this point, we know that 'pid' refers to a real pattern and that
        // 'group_index' refers to a real group. We therefore also know that
        // the pattern and group can be combined to return a correct slot.
        // That's why we don't need to use checked arithmetic below.
        if group_index == 0 {
            Some(pid.as_usize() * 2)
        } else {
            // As above, we don't need to check that our slot is less than the
            // end of our range since we already know the group index is a
            // valid index for the given pattern.
            let (start, _) = self.0.slot_ranges[pid];
            Some(start.as_usize() + ((group_index - 1) * 2))
        }
    }