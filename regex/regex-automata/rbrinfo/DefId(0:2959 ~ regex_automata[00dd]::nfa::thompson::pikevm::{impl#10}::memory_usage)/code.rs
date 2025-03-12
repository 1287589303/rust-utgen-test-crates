fn memory_usage(&self) -> usize {
        self.table.len() * core::mem::size_of::<Option<NonMaxUsize>>()
    }