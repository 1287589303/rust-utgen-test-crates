pub fn dfa_size_limit(mut self, bytes: Option<usize>) -> Config {
        self.dfa_size_limit = Some(bytes);
        self
    }