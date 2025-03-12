fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        // Even if the regex has explicit capture groups, if the caller didn't
        // provide any explicit slots, then it doesn't make sense to try and do
        // extra work to get offsets for those slots. Ideally the caller should
        // realize this and not call this routine in the first place, but alas,
        // we try to save the caller from themselves if they do.
        if !self.is_capture_search_needed(slots.len()) {
            trace!("asked for slots unnecessarily, trying fast path");
            let m = self.search(cache, input)?;
            copy_match_to_slots(m, slots);
            return Some(m.pattern());
        }
        // If the onepass DFA is available for this search (which only happens
        // when it's anchored), then skip running a fallible DFA. The onepass
        // DFA isn't as fast as a full or lazy DFA, but it is typically quite
        // a bit faster than the backtracker or the PikeVM. So it isn't as
        // advantageous to try and do a full/lazy DFA scan first.
        //
        // We still theorize that it's better to do a full/lazy DFA scan, even
        // when it's anchored, because it's usually much faster and permits us
        // to say "no match" much more quickly. This does hurt the case of,
        // say, parsing each line in a log file into capture groups, because
        // in that case, the line always matches. So the lazy DFA scan is
        // usually just wasted work. But, the lazy DFA is usually quite fast
        // and doesn't cost too much here.
        if self.onepass.get(&input).is_some() {
            return self.search_slots_nofail(cache, &input, slots);
        }
        let m = match self.try_search_mayfail(cache, input) {
            Some(Ok(Some(m))) => m,
            Some(Ok(None)) => return None,
            Some(Err(_err)) => {
                trace!("fast capture search failed: {}", _err);
                return self.search_slots_nofail(cache, input, slots);
            }
            None => {
                return self.search_slots_nofail(cache, input, slots);
            }
        };
        // At this point, now that we've found the bounds of the
        // match, we need to re-run something that can resolve
        // capturing groups. But we only need to run on it on the
        // match bounds and not the entire haystack.
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
        Some(
            self.search_slots_nofail(cache, &input, slots)
                .expect("should find a match"),
        )
    }