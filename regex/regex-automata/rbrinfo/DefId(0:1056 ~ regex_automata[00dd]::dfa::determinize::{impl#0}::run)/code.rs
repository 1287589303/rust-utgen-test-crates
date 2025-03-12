pub fn run(
        &self,
        nfa: &thompson::NFA,
        dfa: &mut dense::OwnedDFA,
    ) -> Result<(), BuildError> {
        let dead = State::dead();
        let quit = State::dead();
        let mut cache = StateMap::default();
        // We only insert the dead state here since its representation is
        // identical to the quit state. And we never want anything pointing
        // to the quit state other than specific transitions derived from the
        // determinizer's configured "quit" bytes.
        //
        // We do put the quit state into 'builder_states' below. This ensures
        // that a proper DFA state ID is allocated for it, and that no other
        // DFA state uses the "location after the DEAD state." That is, it
        // is assumed that the quit state is always the state immediately
        // following the DEAD state.
        cache.insert(dead.clone(), DEAD);

        let runner = Runner {
            config: self.clone(),
            nfa,
            dfa,
            builder_states: alloc::vec![dead, quit],
            cache,
            memory_usage_state: 0,
            sparses: SparseSets::new(nfa.states().len()),
            stack: alloc::vec![],
            scratch_state_builder: StateBuilderEmpty::new(),
        };
        runner.run()
    }