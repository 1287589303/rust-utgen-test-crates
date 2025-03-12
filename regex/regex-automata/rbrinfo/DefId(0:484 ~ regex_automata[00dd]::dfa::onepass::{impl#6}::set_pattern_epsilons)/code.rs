fn set_pattern_epsilons(&mut self, sid: StateID, pateps: PatternEpsilons) {
        let offset = sid.as_usize() << self.stride2();
        self.table[offset + self.pateps_offset] = Transition(pateps.0);
    }