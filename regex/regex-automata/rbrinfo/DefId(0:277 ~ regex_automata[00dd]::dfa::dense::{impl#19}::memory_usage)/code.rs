fn memory_usage(&self) -> usize {
        self.table().len() * StateID::SIZE
    }