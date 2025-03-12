pub fn memory_usage(&self) -> usize {
        self.stack.len() * core::mem::size_of::<Frame>()
            + self.visited.memory_usage()
    }