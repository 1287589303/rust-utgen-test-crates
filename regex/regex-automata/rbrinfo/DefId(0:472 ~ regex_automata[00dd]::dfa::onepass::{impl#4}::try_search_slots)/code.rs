pub fn try_search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<PatternID>, MatchError> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        if !utf8empty {
            return self.try_search_slots_imp(cache, input, slots);
        }
        // See PikeVM::try_search_slots for why we do this.
        let min = self.get_nfa().group_info().implicit_slot_len();
        if slots.len() >= min {
            return self.try_search_slots_imp(cache, input, slots);
        }
        if self.get_nfa().pattern_len() == 1 {
            let mut enough = [None, None];
            let got = self.try_search_slots_imp(cache, input, &mut enough)?;
            // This is OK because we know `enough_slots` is strictly bigger
            // than `slots`, otherwise this special case isn't reached.
            slots.copy_from_slice(&enough[..slots.len()]);
            return Ok(got);
        }
        let mut enough = vec![None; min];
        let got = self.try_search_slots_imp(cache, input, &mut enough)?;
        // This is OK because we know `enough_slots` is strictly bigger than
        // `slots`, otherwise this special case isn't reached.
        slots.copy_from_slice(&enough[..slots.len()]);
        Ok(got)
    }