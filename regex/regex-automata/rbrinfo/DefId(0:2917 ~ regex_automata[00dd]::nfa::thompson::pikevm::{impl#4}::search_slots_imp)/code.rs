fn search_slots_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        let hm = match self.search_imp(cache, input, slots) {
            None => return None,
            Some(hm) if !utf8empty => return Some(hm),
            Some(hm) => hm,
        };
        empty::skip_splits_fwd(input, hm, hm.offset(), |input| {
            Ok(self
                .search_imp(cache, input, slots)
                .map(|hm| (hm, hm.offset())))
        })
        // OK because the PikeVM never errors.
        .unwrap()
    }