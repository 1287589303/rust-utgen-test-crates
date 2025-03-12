fn which_overlapping_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        // NOTE: This is effectively a copy of 'search_imp' above, but with no
        // captures support and instead writes patterns that matched directly
        // to 'patset'. See that routine for better commentary about what's
        // going on in this routine. We probably could unify the routines using
        // generics or more helper routines, but I'm not sure it's worth it.
        //
        // NOTE: We somewhat go out of our way here to support things like
        // 'input.get_earliest()' and 'leftmost-first' match semantics. Neither
        // of those seem particularly relevant to this routine, but they are
        // both supported by the DFA analogs of this routine by construction
        // and composition, so it seems like good sense to have the PikeVM
        // match that behavior.

        cache.setup_search(0);
        if input.is_done() {
            return;
        }
        assert!(
            input.haystack().len() < core::usize::MAX,
            "byte slice lengths must be less than usize MAX",
        );
        instrument!(|c| c.reset(&self.nfa));

        let allmatches =
            self.config.get_match_kind().continue_past_first_match();
        let (anchored, start_id) = match self.start_config(input) {
            None => return,
            Some(config) => config,
        };

        let Cache { ref mut stack, ref mut curr, ref mut next } = cache;
        for at in input.start()..=input.end() {
            let any_matches = !patset.is_empty();
            if curr.set.is_empty() {
                if any_matches && !allmatches {
                    break;
                }
                if anchored && at > input.start() {
                    break;
                }
            }
            if !any_matches || allmatches {
                let slots = &mut [];
                self.epsilon_closure(stack, slots, curr, input, at, start_id);
            }
            self.nexts_overlapping(stack, curr, next, input, at, patset);
            // If we found a match and filled our set, then there is no more
            // additional info that we can provide. Thus, we can quit. We also
            // quit if the caller asked us to stop at the earliest point that
            // we know a match exists.
            if patset.is_full() || input.get_earliest() {
                break;
            }
            core::mem::swap(curr, next);
            next.set.clear();
        }
        instrument!(|c| c.eprint(&self.nfa));
    }