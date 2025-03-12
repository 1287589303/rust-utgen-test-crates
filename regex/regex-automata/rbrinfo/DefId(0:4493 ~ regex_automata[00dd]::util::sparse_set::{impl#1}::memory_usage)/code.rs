pub(crate) fn memory_usage(&self) -> usize {
        self.dense.len() * StateID::SIZE + self.sparse.len() * StateID::SIZE
    }