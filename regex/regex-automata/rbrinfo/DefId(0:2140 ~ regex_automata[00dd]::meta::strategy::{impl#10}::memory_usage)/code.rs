fn memory_usage(&self) -> usize {
        self.core.memory_usage()
            + self.preinner.memory_usage()
            + self.nfarev.memory_usage()
            + self.dfa.memory_usage()
    }