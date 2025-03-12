pub fn memory_usage(&self) -> usize {
        use core::mem::size_of;

        self.table.len() * size_of::<Transition>()
            + self.starts.len() * size_of::<StateID>()
    }