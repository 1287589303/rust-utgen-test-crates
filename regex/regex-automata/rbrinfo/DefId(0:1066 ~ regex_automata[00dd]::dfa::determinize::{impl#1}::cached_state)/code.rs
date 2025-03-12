fn cached_state(
        &mut self,
        dfa_id: StateID,
        unit: alphabet::Unit,
    ) -> Result<(StateID, bool), BuildError> {
        // Compute the set of all reachable NFA states, including epsilons.
        let empty_builder = self.get_state_builder();
        let builder = util::determinize::next(
            self.nfa,
            self.config.match_kind,
            &mut self.sparses,
            &mut self.stack,
            &self.builder_states[self.dfa.to_index(dfa_id)],
            unit,
            empty_builder,
        );
        self.maybe_add_state(builder)
    }