pub fn match_pattern(
        &self,
        cache: &Cache,
        id: LazyStateID,
        match_index: usize,
    ) -> PatternID {
        // This is an optimization for the very common case of a DFA with a
        // single pattern. This conditional avoids a somewhat more costly path
        // that finds the pattern ID from the corresponding `State`, which
        // requires a bit of slicing/pointer-chasing. This optimization tends
        // to only matter when matches are frequent.
        if self.pattern_len() == 1 {
            return PatternID::ZERO;
        }
        LazyRef::new(self, cache)
            .get_cached_state(id)
            .match_pattern(match_index)
    }