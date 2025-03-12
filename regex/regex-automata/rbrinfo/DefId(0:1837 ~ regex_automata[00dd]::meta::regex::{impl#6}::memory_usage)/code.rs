pub(crate) fn memory_usage(&self) -> usize {
        self.props().iter().map(|p| p.memory_usage()).sum::<usize>()
            + self.props_union().memory_usage()
    }