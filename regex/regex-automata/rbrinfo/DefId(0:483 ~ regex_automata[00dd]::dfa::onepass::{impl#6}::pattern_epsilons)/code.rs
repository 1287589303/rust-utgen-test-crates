fn pattern_epsilons(&self, sid: StateID) -> PatternEpsilons {
        let offset = sid.as_usize() << self.stride2();
        PatternEpsilons(self.table[offset + self.pateps_offset].0)
    }