fn nexts_overlapping(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        patset: &mut PatternSet,
    ) {
        instrument!(|c| c.record_state_set(&curr.set));
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        let ActiveStates { ref set, ref mut slot_table } = *curr;
        for sid in set.iter() {
            let pid = match self.next(stack, slot_table, next, input, at, sid)
            {
                None => continue,
                Some(pid) => pid,
            };
            // This handles the case of finding a zero-width match that splits
            // a codepoint. Namely, if we're in UTF-8 mode AND we know we can
            // match the empty string, then the only valid way of getting to
            // this point with an offset that splits a codepoint is when we
            // have an empty match. Such matches, in UTF-8 mode, must not be
            // reported. So we just skip them here and pretend as if we did
            // not see a match.
            if utf8empty && !input.is_char_boundary(at) {
                continue;
            }
            let _ = patset.try_insert(pid);
            if !self.config.get_match_kind().continue_past_first_match() {
                break;
            }
        }
    }