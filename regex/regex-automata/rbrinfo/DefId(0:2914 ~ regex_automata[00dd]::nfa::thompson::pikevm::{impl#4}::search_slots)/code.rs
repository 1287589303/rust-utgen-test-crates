pub fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        if !utf8empty {
            let hm = self.search_slots_imp(cache, input, slots)?;
            return Some(hm.pattern());
        }
        // There is an unfortunate special case where if the regex can
        // match the empty string and UTF-8 mode is enabled, the search
        // implementation requires that the slots have at least as much space
        // to report the bounds of any match. This is so zero-width matches
        // that split a codepoint can be filtered out.
        //
        // Note that if utf8empty is true, we specialize the case for when
        // the number of patterns is 1. In that case, we can just use a stack
        // allocation. Otherwise we resort to a heap allocation, which we
        // convince ourselves we're fine with due to the pathological nature of
        // this case.
        let min = self.get_nfa().group_info().implicit_slot_len();
        if slots.len() >= min {
            let hm = self.search_slots_imp(cache, input, slots)?;
            return Some(hm.pattern());
        }
        if self.get_nfa().pattern_len() == 1 {
            let mut enough = [None, None];
            let got = self.search_slots_imp(cache, input, &mut enough);
            // This is OK because we know `enough` is strictly bigger than
            // `slots`, otherwise this special case isn't reached.
            slots.copy_from_slice(&enough[..slots.len()]);
            return got.map(|hm| hm.pattern());
        }
        let mut enough = vec![None; min];
        let got = self.search_slots_imp(cache, input, &mut enough);
        // This is OK because we know `enough` is strictly bigger than `slots`,
        // otherwise this special case isn't reached.
        slots.copy_from_slice(&enough[..slots.len()]);
        got.map(|hm| hm.pattern())
    }