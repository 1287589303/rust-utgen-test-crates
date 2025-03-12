pub(crate) fn memory_usage(&self) -> usize {
        self.0.as_ref().map_or(0, |e| e.memory_usage())
    }