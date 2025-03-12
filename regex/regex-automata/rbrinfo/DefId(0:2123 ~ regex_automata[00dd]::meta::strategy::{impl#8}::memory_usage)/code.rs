fn memory_usage(&self) -> usize {
        self.core.memory_usage() + self.pre.memory_usage()
    }