fn nexts(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {
        let ActiveStates { ref set, ref mut slot_table } = *curr;
        for sid in set.iter() {
            if self.next(
                stack, slot_table, next, haystack, at, at_ch, at_len, sid,
            ) {
                slots.copy_from_slice(slot_table.for_state(sid));
                return true;
            }
        }
        false
    }