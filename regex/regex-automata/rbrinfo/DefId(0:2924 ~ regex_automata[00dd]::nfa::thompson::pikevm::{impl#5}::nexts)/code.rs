fn nexts(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        instrument!(|c| c.record_state_set(&curr.set));
        let mut pid = None;
        let ActiveStates { ref set, ref mut slot_table } = *curr;
        for sid in set.iter() {
            pid = match self.next(stack, slot_table, next, input, at, sid) {
                None => continue,
                Some(pid) => Some(pid),
            };
            slots.copy_from_slice(slot_table.for_state(sid));
            if !self.config.get_match_kind().continue_past_first_match() {
                break;
            }
        }
        pid
    }