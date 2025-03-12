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
        let m = match self.try_search_full(cache, input) {
            Err(RetryError::Quadratic(_err)) => {
                trace!("reverse inner captures optimization failed: {}", _err);
                return self.core.search_slots(cache, input, slots);
            }
            Err(RetryError::Fail(_err)) => {
                trace!("reverse inner fast captures search failed: {}", _err);
                return self.core.search_slots_nofail(cache, input, slots);
            }
            Ok(None) => return None,
            Ok(Some(m)) => m,
        };
        trace!(
            "match found at {}..{} in capture search, \
		  	 using another engine to find captures",
            m.start(),
            m.end(),
        );
        let input = input
            .clone()
            .span(m.start()..m.end())
            .anchored(Anchored::Pattern(m.pattern()));
        self.core.search_slots_nofail(cache, &input, slots)
    }