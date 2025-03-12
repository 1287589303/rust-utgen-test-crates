fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
        let state_limit = Transition::STATE_ID_LIMIT;
        // Note that unlike dense and lazy DFAs, we specifically do NOT
        // premultiply our state IDs here. The reason is that we want to pack
        // our state IDs into 64-bit transitions with other info, so the fewer
        // the bits we use for state IDs the better. If we premultiply, then
        // our state ID space shrinks. We justify this by the assumption that
        // a one-pass DFA is just already doing a fair bit more work than a
        // normal DFA anyway, so an extra multiplication to compute a state
        // transition doesn't seem like a huge deal.
        let next_id = self.dfa.table.len() >> self.dfa.stride2();
        let id = StateID::new(next_id)
            .map_err(|_| BuildError::too_many_states(state_limit))?;
        if id.as_u64() > Transition::STATE_ID_LIMIT {
            return Err(BuildError::too_many_states(state_limit));
        }
        self.dfa
            .table
            .extend(core::iter::repeat(Transition(0)).take(self.dfa.stride()));
        // The default empty value for 'PatternEpsilons' is sadly not all
        // zeroes. Instead, a special sentinel is used to indicate that there
        // is no pattern. So we need to explicitly set the pattern epsilons to
        // the correct "empty" PatternEpsilons.
        self.dfa.set_pattern_epsilons(id, PatternEpsilons::empty());
        if let Some(size_limit) = self.config.get_size_limit() {
            if self.dfa.memory_usage() > size_limit {
                return Err(BuildError::exceeded_size_limit(size_limit));
            }
        }
        Ok(id)
    }