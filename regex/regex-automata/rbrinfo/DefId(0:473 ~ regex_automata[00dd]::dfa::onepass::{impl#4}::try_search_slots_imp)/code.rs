fn try_search_slots_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<PatternID>, MatchError> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        match self.search_imp(cache, input, slots)? {
            None => return Ok(None),
            Some(pid) if !utf8empty => return Ok(Some(pid)),
            Some(pid) => {
                // These slot indices are always correct because we know our
                // 'pid' is valid and thus we know that the slot indices for it
                // are valid.
                let slot_start = pid.as_usize().wrapping_mul(2);
                let slot_end = slot_start.wrapping_add(1);
                // OK because we know we have a match and we know our caller
                // provided slots are big enough (which we make true above if
                // the caller didn't). Namely, we're only here when 'utf8empty'
                // is true, and when that's true, we require slots for every
                // pattern.
                let start = slots[slot_start].unwrap().get();
                let end = slots[slot_end].unwrap().get();
                // If our match splits a codepoint, then we cannot report is
                // as a match. And since one-pass DFAs only support anchored
                // searches, we don't try to skip ahead to find the next match.
                // We can just quit with nothing.
                if start == end && !input.is_char_boundary(start) {
                    return Ok(None);
                }
                Ok(Some(pid))
            }
        }
    }