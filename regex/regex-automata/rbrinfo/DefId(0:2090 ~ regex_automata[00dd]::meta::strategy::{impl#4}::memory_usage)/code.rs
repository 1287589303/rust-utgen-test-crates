fn memory_usage(&self) -> usize {
        self.info.memory_usage()
            + self.pre.as_ref().map_or(0, |pre| pre.memory_usage())
            + self.nfa.memory_usage()
            + self.nfarev.as_ref().map_or(0, |nfa| nfa.memory_usage())
            + self.onepass.memory_usage()
            + self.dfa.memory_usage()
    }