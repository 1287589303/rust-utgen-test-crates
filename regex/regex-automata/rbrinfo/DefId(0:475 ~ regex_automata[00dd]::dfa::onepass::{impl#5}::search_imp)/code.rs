fn search_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<PatternID>, MatchError> {
        // PERF: Some ideas. I ran out of steam after my initial impl to try
        // many of these.
        //
        // 1) Try doing more state shuffling. Right now, all we do is push
        // match states to the end of the transition table so that we can do
        // 'if sid >= self.min_match_id' to know whether we're in a match
        // state or not. But what about doing something like dense DFAs and
        // pushing dead, match and states with captures/looks all toward the
        // beginning of the transition table. Then we could do 'if sid <=
        // self.max_special_id', in which case, we need to do some special
        // handling of some sort. Otherwise, we get the happy path, just
        // like in a DFA search. The main argument against this is that the
        // one-pass DFA is likely to be used most often with capturing groups
        // and if capturing groups are common, then this might wind up being a
        // pessimization.
        //
        // 2) Consider moving 'PatternEpsilons' out of the transition table.
        // It is only needed for match states and usually a small minority of
        // states are match states. Therefore, we're using an extra 'u64' for
        // most states.
        //
        // 3) I played around with the match state handling and it seems like
        // there is probably a lot left on the table for improvement. The
        // key tension is that the 'find_match' routine is a giant mess, but
        // splitting it out into a non-inlineable function is a non-starter
        // because the match state might consume input, so 'find_match' COULD
        // be called quite a lot, and a function call at that point would trash
        // perf. In theory, we could detect whether a match state consumes
        // input and then specialize our search routine based on that. In that
        // case, maybe an extra function call is OK, but even then, it might be
        // too much of a latency hit. Another idea is to just try and figure
        // out how to reduce the code size of 'find_match'. RE2 has a trick
        // here where the match handling isn't done if we know the next byte of
        // input yields a match too. Maybe we adopt that?
        //
        // This just might be a tricky DFA to optimize.

        if input.is_done() {
            return Ok(None);
        }
        // We unfortunately have a bit of book-keeping to do to set things
        // up. We do have to setup our cache and clear all of our slots. In
        // particular, clearing the slots is necessary for the case where we
        // report a match, but one of the capturing groups didn't participate
        // in the match but had a span set from a previous search. That would
        // be bad. In theory, we could avoid all this slot clearing if we knew
        // that every slot was always activated for every match. Then we would
        // know they would always be overwritten when a match is found.
        let explicit_slots_len = core::cmp::min(
            Slots::LIMIT,
            slots.len().saturating_sub(self.explicit_slot_start),
        );
        cache.setup_search(explicit_slots_len);
        for slot in cache.explicit_slots() {
            *slot = None;
        }
        for slot in slots.iter_mut() {
            *slot = None;
        }
        // We set the starting slots for every pattern up front. This does
        // increase our latency somewhat, but it avoids having to do it every
        // time we see a match state (which could be many times in a single
        // search if the match state consumes input).
        for pid in self.nfa.patterns() {
            let i = pid.as_usize() * 2;
            if i >= slots.len() {
                break;
            }
            slots[i] = NonMaxUsize::new(input.start());
        }
        let mut pid = None;
        let mut next_sid = match input.get_anchored() {
            Anchored::Yes => self.start(),
            Anchored::Pattern(pid) => self.start_pattern(pid)?,
            Anchored::No => {
                // If the regex is itself always anchored, then we're fine,
                // even if the search is configured to be unanchored.
                if !self.nfa.is_always_start_anchored() {
                    return Err(MatchError::unsupported_anchored(
                        Anchored::No,
                    ));
                }
                self.start()
            }
        };
        let leftmost_first =
            matches!(self.config.get_match_kind(), MatchKind::LeftmostFirst);
        for at in input.start()..input.end() {
            let sid = next_sid;
            let trans = self.transition(sid, input.haystack()[at]);
            next_sid = trans.state_id();
            let epsilons = trans.epsilons();
            if sid >= self.min_match_id {
                if self.find_match(cache, input, at, sid, slots, &mut pid) {
                    if input.get_earliest()
                        || (leftmost_first && trans.match_wins())
                    {
                        return Ok(pid);
                    }
                }
            }
            if sid == DEAD
                || (!epsilons.looks().is_empty()
                    && !self.nfa.look_matcher().matches_set_inline(
                        epsilons.looks(),
                        input.haystack(),
                        at,
                    ))
            {
                return Ok(pid);
            }
            epsilons.slots().apply(at, cache.explicit_slots());
        }
        if next_sid >= self.min_match_id {
            self.find_match(
                cache,
                input,
                input.end(),
                next_sid,
                slots,
                &mut pid,
            );
        }
        Ok(pid)
    }