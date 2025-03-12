fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        if input.get_anchored().is_anchored() {
            return self.core.search_slots(cache, input, slots);
        }
        match self.try_search_half_anchored_rev(cache, input) {
            Err(_err) => {
                trace!("fast reverse anchored search failed: {}", _err);
                self.core.search_slots_nofail(cache, input, slots)
            }
            Ok(None) => None,
            Ok(Some(hm)) => {
                if !self.core.is_capture_search_needed(slots.len()) {
                    trace!("asked for slots unnecessarily, skipping captures");
                    let m = Match::new(hm.pattern(), hm.offset()..input.end());
                    copy_match_to_slots(m, slots);
                    return Some(m.pattern());
                }
                let start = hm.offset();
                let input = input
                    .clone()
                    .span(start..input.end())
                    .anchored(Anchored::Pattern(hm.pattern()));
                self.core.search_slots_nofail(cache, &input, slots)
            }
        }
    }