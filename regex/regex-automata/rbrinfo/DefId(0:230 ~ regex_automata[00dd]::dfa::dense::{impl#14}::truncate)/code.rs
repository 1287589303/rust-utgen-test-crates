fn truncate(&mut self, len: usize) {
        self.table.truncate(len << self.stride2);
    }