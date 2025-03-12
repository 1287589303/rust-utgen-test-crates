fn memory_usage(&self) -> usize {
        self.bitset.len() * core::mem::size_of::<usize>()
    }