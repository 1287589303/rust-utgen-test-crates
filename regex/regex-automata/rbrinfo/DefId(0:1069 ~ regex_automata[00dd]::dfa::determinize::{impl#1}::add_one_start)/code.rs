fn add_one_start(
        &mut self,
        nfa_start: StateID,
        start: Start,
    ) -> Result<(StateID, bool), BuildError> {
        // Compute the look-behind assertions that are true in this starting
        // configuration, and the determine the epsilon closure. While
        // computing the epsilon closure, we only follow condiional epsilon
        // transitions that satisfy the look-behind assertions in 'look_have'.
        let mut builder_matches = self.get_state_builder().into_matches();
        util::determinize::set_lookbehind_from_start(
            self.nfa,
            &start,
            &mut builder_matches,
        );
        self.sparses.set1.clear();
        util::determinize::epsilon_closure(
            self.nfa,
            nfa_start,
            builder_matches.look_have(),
            &mut self.stack,
            &mut self.sparses.set1,
        );
        let mut builder = builder_matches.into_nfa();
        util::determinize::add_nfa_states(
            &self.nfa,
            &self.sparses.set1,
            &mut builder,
        );
        self.maybe_add_state(builder)
    }