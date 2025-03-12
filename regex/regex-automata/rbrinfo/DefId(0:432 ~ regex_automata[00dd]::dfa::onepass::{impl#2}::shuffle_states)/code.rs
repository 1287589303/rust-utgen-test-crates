fn shuffle_states(&mut self) {
        let mut remapper = Remapper::new(&self.dfa);
        let mut next_dest = self.dfa.last_state_id();
        for i in (0..self.dfa.state_len()).rev() {
            let id = StateID::must(i);
            let is_match =
                self.dfa.pattern_epsilons(id).pattern_id().is_some();
            if !is_match {
                continue;
            }
            remapper.swap(&mut self.dfa, next_dest, id);
            self.dfa.min_match_id = next_dest;
            next_dest = self.dfa.prev_state_id(next_dest).expect(
                "match states should be a proper subset of all states",
            );
        }
        remapper.remap(&mut self.dfa);
    }