fn memory_usage(&self) -> usize {
        (self.states.len() * size_of::<State>())
            + (self.cap_index_to_name.len() * size_of::<Option<Arc<str>>>())
            + self.memory_extra
    }