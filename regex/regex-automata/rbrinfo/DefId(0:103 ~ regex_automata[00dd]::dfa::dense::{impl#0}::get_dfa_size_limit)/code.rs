pub fn get_dfa_size_limit(&self) -> Option<usize> {
        self.dfa_size_limit.unwrap_or(None)
    }