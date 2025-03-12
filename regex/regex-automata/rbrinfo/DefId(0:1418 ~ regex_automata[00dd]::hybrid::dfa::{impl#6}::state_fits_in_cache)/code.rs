fn state_fits_in_cache(&self, state: &State) -> bool {
        let needed = self.cache.memory_usage()
            + self.memory_usage_for_one_more_state(state.memory_usage());
        trace!(
            "lazy DFA cache capacity check: {:?} ?<=? {:?}",
            needed,
            self.dfa.cache_capacity
        );
        needed <= self.dfa.cache_capacity
    }