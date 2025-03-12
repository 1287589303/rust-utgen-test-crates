fn search_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<HalfMatch>, MatchError> {
        // Unlike in the PikeVM, we write our capturing group spans directly
        // into the caller's captures groups. So we have to make sure we're
        // starting with a blank slate first. In the PikeVM, we avoid this
        // by construction: the spans that are copied to every slot in the
        // 'Captures' value already account for presence/absence. In this
        // backtracker, we write directly into the caller provided slots, where
        // as in the PikeVM, we write into scratch space first and only copy
        // them to the caller provided slots when a match is found.
        for slot in slots.iter_mut() {
            *slot = None;
        }
        cache.setup_search(&self, input)?;
        if input.is_done() {
            return Ok(None);
        }
        let (anchored, start_id) = match input.get_anchored() {
            // Only way we're unanchored is if both the caller asked for an
            // unanchored search *and* the pattern is itself not anchored.
            Anchored::No => (
                self.nfa.is_always_start_anchored(),
                // We always use the anchored starting state here, even if
                // doing an unanchored search. The "unanchored" part of it is
                // implemented in the loop below, by simply trying the next
                // byte offset if the previous backtracking exploration failed.
                self.nfa.start_anchored(),
            ),
            Anchored::Yes => (true, self.nfa.start_anchored()),
            Anchored::Pattern(pid) => match self.nfa.start_pattern(pid) {
                None => return Ok(None),
                Some(sid) => (true, sid),
            },
        };
        if anchored {
            let at = input.start();
            return Ok(self.backtrack(cache, input, at, start_id, slots));
        }
        let pre = self.get_config().get_prefilter();
        let mut at = input.start();
        while at <= input.end() {
            if let Some(ref pre) = pre {
                let span = Span::from(at..input.end());
                match pre.find(input.haystack(), span) {
                    None => break,
                    Some(ref span) => at = span.start,
                }
            }
            if let Some(hm) = self.backtrack(cache, input, at, start_id, slots)
            {
                return Ok(Some(hm));
            }
            at += 1;
        }
        Ok(None)
    }