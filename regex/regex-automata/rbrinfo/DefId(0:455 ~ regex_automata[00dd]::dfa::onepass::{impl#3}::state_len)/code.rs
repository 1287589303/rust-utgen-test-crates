pub fn state_len(&self) -> usize {
        self.table.len() >> self.stride2()
    }