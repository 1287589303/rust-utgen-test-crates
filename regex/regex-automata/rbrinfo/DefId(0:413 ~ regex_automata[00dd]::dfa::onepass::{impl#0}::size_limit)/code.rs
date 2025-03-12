pub fn size_limit(mut self, limit: Option<usize>) -> Config {
        self.size_limit = Some(limit);
        self
    }