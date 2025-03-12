pub fn determinize_size_limit(mut self, bytes: Option<usize>) -> Config {
        self.determinize_size_limit = Some(bytes);
        self
    }