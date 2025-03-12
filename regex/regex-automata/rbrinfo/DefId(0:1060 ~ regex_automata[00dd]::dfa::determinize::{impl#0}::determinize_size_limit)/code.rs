pub fn determinize_size_limit(
        &mut self,
        bytes: Option<usize>,
    ) -> &mut Config {
        self.determinize_size_limit = bytes;
        self
    }