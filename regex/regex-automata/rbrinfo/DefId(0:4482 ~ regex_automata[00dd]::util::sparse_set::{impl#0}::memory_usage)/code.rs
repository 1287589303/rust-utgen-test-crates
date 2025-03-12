pub(crate) fn memory_usage(&self) -> usize {
        self.set1.memory_usage() + self.set2.memory_usage()
    }