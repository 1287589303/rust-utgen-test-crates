fn size_limit(&mut self, limit: usize) -> &mut Builder {
        self.metac = self.metac.clone().nfa_size_limit(Some(limit));
        self
    }