fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder {
        self.metac = self.metac.clone().hybrid_cache_capacity(limit);
        self
    }