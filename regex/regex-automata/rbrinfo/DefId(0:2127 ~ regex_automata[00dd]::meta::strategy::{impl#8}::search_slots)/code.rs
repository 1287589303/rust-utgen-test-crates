fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        if input.get_anchored().is_anchored() {
            return self.core.search_slots(cache, input, slots);
        }
        if !self.core.is_capture_search_needed(slots.len()) {
            trace!("asked for slots unnecessarily, trying fast path");
            let m = self.search(cache, input)?;
            copy_match_to_slots(m, slots);
            return Some(m.pattern());
        }
        let hm_start = match self.try_search_half_start(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!(
                    "reverse suffix captures optimization failed: {}",
                    _err
                );
                return self.core.search_slots(cache, input, slots);
            }
            Err(RetryError::Fail(_err)) => {
                trace!(
                    "reverse suffix reverse fast captures search failed: {}",
                    _err
                );
                return self.core.search_slots_nofail(cache, input, slots);
            }
            Ok(None) => return None,
            Ok(Some(hm_start)) => hm_start,
        };
        trace!(
            "match found at {}..{} in capture search, \
		  	 using another engine to find captures",
            hm_start.offset(),
            input.end(),
        );
        let start = hm_start.offset();
        let input = input
            .clone()
            .span(start..input.end())
            .anchored(Anchored::Pattern(hm_start.pattern()));
        self.core.search_slots_nofail(cache, &input, slots)
    }