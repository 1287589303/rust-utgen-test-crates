fn find_match(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        at: usize,
        sid: StateID,
        slots: &mut [Option<NonMaxUsize>],
        matched_pid: &mut Option<PatternID>,
    ) -> bool {
        debug_assert!(sid >= self.min_match_id);
        let pateps = self.pattern_epsilons(sid);
        let epsilons = pateps.epsilons();
        if !epsilons.looks().is_empty()
            && !self.nfa.look_matcher().matches_set_inline(
                epsilons.looks(),
                input.haystack(),
                at,
            )
        {
            return false;
        }
        let pid = pateps.pattern_id_unchecked();
        // This calculation is always correct because we know our 'pid' is
        // valid and thus we know that the slot indices for it are valid.
        let slot_end = pid.as_usize().wrapping_mul(2).wrapping_add(1);
        // Set the implicit 'end' slot for the matching pattern. (The 'start'
        // slot was set at the beginning of the search.)
        if slot_end < slots.len() {
            slots[slot_end] = NonMaxUsize::new(at);
        }
        // If the caller provided enough room, copy the previously recorded
        // explicit slots from our scratch space to the caller provided slots.
        // We *also* need to set any explicit slots that are active as part of
        // the path to the match state.
        if self.explicit_slot_start < slots.len() {
            // NOTE: The 'cache.explicit_slots()' slice is setup at the
            // beginning of every search such that it is guaranteed to return a
            // slice of length equivalent to 'slots[explicit_slot_start..]'.
            slots[self.explicit_slot_start..]
                .copy_from_slice(cache.explicit_slots());
            epsilons.slots().apply(at, &mut slots[self.explicit_slot_start..]);
        }
        *matched_pid = Some(pid);
        true
    }