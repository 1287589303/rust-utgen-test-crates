fn reset_cache(&mut self) {
        self.cache.state_saver = StateSaver::none();
        self.clear_cache();
        // If a new DFA is used, it might have a different number of NFA
        // states, so we need to make sure our sparse sets have the appropriate
        // size.
        self.cache.sparses.resize(self.dfa.get_nfa().states().len());
        self.cache.clear_count = 0;
        self.cache.progress = None;
    }