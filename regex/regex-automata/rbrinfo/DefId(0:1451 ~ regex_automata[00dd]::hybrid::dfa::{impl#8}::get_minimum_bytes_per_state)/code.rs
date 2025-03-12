pub fn get_minimum_bytes_per_state(&self) -> Option<usize> {
        self.minimum_bytes_per_state.unwrap_or(None)
    }