pub fn minimum_bytes_per_state(mut self, min: Option<usize>) -> Config {
        self.minimum_bytes_per_state = Some(min);
        self
    }