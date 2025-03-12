fn memory_usage(&self) -> usize {
        use core::mem::size_of;

        self.builder_states.len() * size_of::<State>()
        // Maps likely use more memory than this, but it's probably close.
        + self.cache.len() * (size_of::<State>() + size_of::<StateID>())
        + self.memory_usage_state
        + self.stack.capacity() * size_of::<StateID>()
        + self.scratch_state_builder.capacity()
    }