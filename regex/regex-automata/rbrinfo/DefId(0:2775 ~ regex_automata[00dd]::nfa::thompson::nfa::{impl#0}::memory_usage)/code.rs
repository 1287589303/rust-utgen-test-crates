pub fn memory_usage(&self) -> usize {
        use core::mem::size_of;

        size_of::<Inner>() // allocated on the heap via Arc
            + self.0.states.len() * size_of::<State>()
            + self.0.start_pattern.len() * size_of::<StateID>()
            + self.0.group_info.memory_usage()
            + self.0.memory_extra
    }