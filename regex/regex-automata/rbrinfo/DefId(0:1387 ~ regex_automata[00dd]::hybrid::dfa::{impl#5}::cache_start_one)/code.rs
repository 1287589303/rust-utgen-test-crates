fn cache_start_one(
        &mut self,
        nfa_start_id: NFAStateID,
        start: Start,
    ) -> Result<LazyStateID, CacheError> {
        let mut builder_matches = self.get_state_builder().into_matches();
        determinize::set_lookbehind_from_start(
            self.dfa.get_nfa(),
            &start,
            &mut builder_matches,
        );
        self.cache.sparses.set1.clear();
        determinize::epsilon_closure(
            self.dfa.get_nfa(),
            nfa_start_id,
            builder_matches.look_have(),
            &mut self.cache.stack,
            &mut self.cache.sparses.set1,
        );
        let mut builder = builder_matches.into_nfa();
        determinize::add_nfa_states(
            &self.dfa.get_nfa(),
            &self.cache.sparses.set1,
            &mut builder,
        );
        let tag_starts = self.dfa.get_config().get_specialize_start_states();
        self.add_builder_state(builder, |id| {
            if tag_starts {
                id.to_start()
            } else {
                id
            }
        })
    }