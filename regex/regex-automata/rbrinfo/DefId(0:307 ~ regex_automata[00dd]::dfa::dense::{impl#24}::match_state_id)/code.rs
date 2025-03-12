fn match_state_id(&self, dfa: &DFA<T>, index: usize) -> StateID {
        assert!(dfa.special.matches(), "no match states to index");
        // This is one of the places where we rely on the fact that match
        // states are contiguous in the transition table. Namely, that the
        // first match state ID always corresponds to dfa.special.min_start.
        // From there, since we know the stride, we can compute the ID of any
        // match state given its index.
        let stride2 = u32::try_from(dfa.stride2()).unwrap();
        let offset = index.checked_shl(stride2).unwrap();
        let id = dfa.special.min_match.as_usize().checked_add(offset).unwrap();
        let sid = StateID::new(id).unwrap();
        assert!(dfa.is_match_state(sid));
        sid
    }