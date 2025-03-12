fn match_state_index(&self, id: StateID) -> usize {
        debug_assert!(self.is_match_state(id));
        // This is one of the places where we rely on the fact that match
        // states are contiguous in the transition table. Namely, that the
        // first match state ID always corresponds to dfa.special.min_match.
        // From there, since we know the stride, we can compute the overall
        // index of any match state given the match state's ID.
        let min = self.special().min_match.as_usize();
        // CORRECTNESS: We're allowed to produce an incorrect result or panic,
        // so both the subtraction and the unchecked StateID construction is
        // OK.
        self.to_index(StateID::new_unchecked(id.as_usize() - min))
    }